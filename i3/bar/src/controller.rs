use std::sync::mpsc::{self, Sender};
use std::thread;
use std::io::{self, Write};

use serde_json as json;

use i3status::codec::Element;

pub struct Controller {
    stdout: Sender<String>,

    error: Option<Element>,

    disk_info: Option<Element>,
    networks: Vec<Element>,
    battery: Option<Element>,
    cpu_usage: Option<Element>,
    load: Option<Element>,
    datetime: Option<Element>,
    unknown: Vec<Element>,
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
            datetime: None,
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
        if let Some(e) = self.datetime.as_ref() {
            elements.push(e.clone());
        }
        for unknown in &self.unknown {
            elements.push(unknown.clone());
        }

        let mut line = json::to_string(&elements).unwrap();
        line += ",";
        self.stdout.send(line).unwrap()
    }

    pub fn set_disk_info(&mut self, disk_info: Element) {
        self.disk_info = Some(disk_info);
    }

    pub fn set_networks(&mut self, networks: Vec<Element>) {
        self.networks = networks;
    }

    pub fn set_battery(&mut self, battery: Element) {
        self.battery = Some(battery);
    }

    pub fn set_cpu_usage(&mut self, cpu_usage: Element) {
        self.cpu_usage = Some(cpu_usage);
    }

    pub fn set_load(&mut self, load: Element) {
        self.load = Some(load);
    }

    pub fn set_datetime(&mut self, datetime: Element) {
        self.datetime = Some(datetime);
    }

    pub fn set_unknown(&mut self, unknown: Vec<Element>) {
        self.unknown = unknown;
    }

    pub fn push_error(&mut self, error: String) {
        if let Some(err) = self.error.as_mut() {
            err.full_text.push(';');
            err.full_text.push_str(&error);
            return;
        }
        self.error = Some(Element {
            name: "error".to_string(),
            instance: None,
            markup: "none".to_string(),
            full_text: error,
            color: Some("#FF0000".to_string()),
        });
    }
}