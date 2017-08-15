use futures::sync::mpsc::UnboundedSender;
use tokio_core::reactor::Handle;

use codec::{Block, BlockBuilder, Header, Element};
use stdout;

pub struct Controller {
    stdout: UnboundedSender<Element>,

    error_idx: u64,
    errors: Vec<Block>,

    media: Option<Block>,
    backlight: Option<Block>,
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
    pub fn new(handle: &Handle) -> Controller {
        let send = stdout::stdout(handle);
        // init
        (&send).send(Element::Header(Header {
            version: 1,
            stop_signal: None,
            cont_signal: None,
            click_events: None,
        })).unwrap();
        (&send).send(Element::OpenStream).unwrap();
        Controller {
            stdout: send,
            error_idx: 0,
            errors: Vec::new(),

            media: None,
            backlight: None,
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
        let mut blocks = Vec::new();

        for error in &self.errors {
            blocks.push(error.clone());
        }
        self.media.as_ref().map(|e| blocks.push(e.clone()));
        self.backlight.as_ref().map(|e| blocks.push(e.clone()));
        if let Some(e) = self.disk_info.as_ref() {
            blocks.push(e.clone());
        }
        for network in &self.networks {
            blocks.push(network.clone());
        }
        if let Some(e) = self.battery.as_ref() {
            blocks.push(e.clone());
        }
        if let Some(e) = self.cpu_usage.as_ref() {
            blocks.push(e.clone());
        }
        if let Some(e) = self.load.as_ref() {
            blocks.push(e.clone());
        }
        self.ram.as_ref().map(|e| blocks.push(e.clone()));
        self.swap.as_ref().map(|e| blocks.push(e.clone()));
        self.time.as_ref().map(|e| blocks.push(e.clone()));
        self.date.as_ref().map(|e| blocks.push(e.clone()));
        for unknown in &self.unknown {
            blocks.push(unknown.clone());
        }

        (&self.stdout).send(Element::Blocks(blocks)).unwrap()
    }

    pub fn set_media(&mut self, media: Option<Block>) {
        self.media = media;
    }

    pub fn set_backlight(&mut self, backlight: Block) {
        self.backlight = Some(backlight);
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
        self.errors.push(BlockBuilder::new(error)
            .name(format!("error{}", self.error_idx))
            .color("#FF0000".to_string())
            .build());
        self.error_idx += 1;
    }

    pub fn clear_error(&mut self, name: &str) {
        let pos = self.errors.iter().position(|e| e.name.as_ref().unwrap() == name).unwrap();
        self.errors.remove(pos);
        self.update();
    }
}
