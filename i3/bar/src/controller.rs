use std::sync::mpsc::{self, Sender};
use std::thread;
use std::io::{self, Write};

use serde_json as json;

use codec::{Block, BlockBuilder};

pub struct Controller {
    stdout: Sender<String>,

    error: Option<Block>,

    disk_info: Option<Block>,
    networks: Vec<Block>,
    battery: Option<Block>,
    cpu_usage: Option<Block>,
    load: Option<Block>,
    ram: Option<Block>,
    swap: Option<Block>,
    time: Option<Block>,
    date: Option<Block>,
    unknown: Vec<Block>,
}

impl Controller {
    pub fn new() -> Controller {
        let (send, recv) = mpsc::channel();
        thread::spawn(move || {
            while let Ok(line) = recv.recv() {
                let stdout = io::stdout();
                let mut stdout = stdout.lock();
                writeln!(stdout, "{}", line).unwrap();
            }
        });
        // init
        send.send("{\"version\":1}".to_string()).unwrap();
        send.send("[".to_string()).unwrap();
        Controller {
            stdout: send,
            error: None,

            disk_info: None,
            networks: Vec::new(),
            battery: None,
            cpu_usage: None,
            load: None,
            ram: None,
            swap: None,
            time: None,
            date: None,
            unknown: Vec::new(),
        }
    }

    pub fn update(&self) {
        let mut elements = Vec::new();

        if let Some(err) = self.error.as_ref() {
            elements.push(err.clone());
        }
        if let Some(e) = self.disk_info.as_ref() {
            elements.push(e.clone());
        }
        for network in &self.networks {
            elements.push(network.clone());
        }
        if let Some(e) = self.battery.as_ref() {
            elements.push(e.clone());
        }
        if let Some(e) = self.cpu_usage.as_ref() {
            elements.push(e.clone());
        }
        if let Some(e) = self.load.as_ref() {
            elements.push(e.clone());
        }
        self.ram.as_ref().map(|e| elements.push(e.clone()));
        self.swap.as_ref().map(|e| elements.push(e.clone()));
        self.time.as_ref().map(|e| elements.push(e.clone()));
        self.date.as_ref().map(|e| elements.push(e.clone()));
        for unknown in &self.unknown {
            elements.push(unknown.clone());
        }

        let mut line = json::to_string(&elements).unwrap();
        line += ",";
        self.stdout.send(line).unwrap()
    }

    pub fn set_disk_info(&mut self, disk_info: Block) {
        self.disk_info = Some(disk_info);
    }

    pub fn set_networks(&mut self, networks: Vec<Block>) {
        self.networks = networks;
    }

    pub fn set_battery(&mut self, battery: Block) {
        self.battery = Some(battery);
    }

    pub fn set_cpu_usage(&mut self, cpu_usage: Block) {
        self.cpu_usage = Some(cpu_usage);
    }

    pub fn set_load(&mut self, load: Block) {
        self.load = Some(load);
    }

    pub fn set_ram(&mut self, ram: Block) {
        self.ram = Some(ram);
    }

    pub fn set_swap(&mut self, swap: Block) {
        self.swap = Some(swap);
    }

    pub fn set_time(&mut self, time: Block) {
        self.time = Some(time);
    }

    pub fn set_date(&mut self, date: Block) {
        self.date = Some(date);
    }

    pub fn set_unknown(&mut self, unknown: Vec<Block>) {
        self.unknown = unknown;
    }

    pub fn push_error(&mut self, error: String) {
        if let Some(err) = self.error.as_mut() {
            err.full_text.push(';');
            err.full_text.push_str(&error);
            return;
        }
        self.error = Some(BlockBuilder::new(error)
            .name("error".to_string())
            .color("#FF0000".to_string())
            .build());
    }
}
