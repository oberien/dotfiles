use std::sync::mpsc::{self, Sender};
use std::thread;
use std::io::{self, Write};

use serde_json as json;

use i3status::codec::Element;

pub struct Controller {
    stdout: Sender<String>,
    i3status: Vec<Element>,
    error: Option<String>,
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
            i3status: Vec::new(),
            error: None,
        }
    }

    fn write(&self) {
        let mut line = self.error.clone().unwrap_or_default();
        line += &json::to_string(&self.i3status).unwrap();
        line += ",";
        self.stdout.send(line).unwrap()
    }

    pub fn set_i3status(&mut self, status: Vec<Element>) {
        self.i3status = status;
        self.write();
    }

    pub fn set_error(&mut self, error: String) {
        if let Some(err) = self.error.as_mut() {
            err.push(';');
            err.push_str(&error);
            return;
        }
        self.error = Some(error);
    }
}