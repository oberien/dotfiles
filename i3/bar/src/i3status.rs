use std::process::{Command, Stdio};
use std::cell::RefCell;
use std::rc::Rc;
use std::io::{Error, BufReader};

use futures::{Future, Stream};
use tokio_core::reactor::Handle;
use tokio_process::CommandExt;
use tokio_io::io;

use controller::Controller;
use codec::{Codec, Block, Element};
use icon;

pub fn i3status(controller: Rc<RefCell<Controller>>, handle: &Handle) -> Box<Future<Item=(), Error=Error>> {
    let codec = Rc::new(RefCell::new(Codec::new()));
    let mut cmd = Command::new("i3status")
        .stdout(Stdio::piped())
        .spawn_async(handle)
        .unwrap();
    let stdout = cmd.stdout().take().unwrap();
    let buf = BufReader::new(stdout);
    let lines = io::lines(buf);
    let elements = lines.and_then(move |line| {
        let line = match line.starts_with(',') {
            true => &line[1..],
            false => &line[..],
        };
        codec.borrow_mut().decode_line(line)
    });
    let future = elements.for_each(move |opt| {
        let mut controller = controller.borrow_mut();
        if let Element::Blocks(vec) = opt {
            let mut networks = Vec::new();
            let mut unknown = Vec::new();
            for e in vec {
                let name = e.name.clone();
                match name.unwrap().as_str() {
                    "disk_info" => controller.set_disk_info(disk_info(e)),
                    "ethernet" => networks.push(ethernet(e)),
                    "wireless" => networks.push(wireless(e)),
                    "battery" => match battery(e) {
                        Ok(e) => controller.set_battery(e),
                        Err(e) => controller.push_error(e)
                    },
                    "cpu_usage" => controller.set_cpu_usage(cpu_usage(e)),
                    "load" => controller.set_load(load(e)),
                    "time" => (),
                    _ => unknown.push(e)
                }
            }
            controller.set_networks(networks);
            controller.set_unknown(unknown);
            controller.update();
        }
        Ok(())
    });
    Box::new(future.join(cmd).map(|_| ()))
}

fn disk_info(mut e: Block) -> Block {
    e.full_text = format!("{} {}", icon::MINIDISK, e.full_text);
    e
}

fn ethernet(mut e: Block) -> Block {
    e.full_text = network(&e.full_text, icon::LAN);
    e
}

fn wireless(mut e: Block) -> Block {
    e.full_text = network(&e.full_text, icon::WIFI);
    e
}

fn network(full_text: &str, icon: char) -> String {
    let mut s = icon.to_string();
    match full_text {
        "" => s.push(icon::STRIKETHROUGH),
        t => { s.push(' '); s += t }
    }
    s
}

fn battery(mut e: Block) -> Result<Block, String> {
    if e.full_text == "No battery" {
        e.full_text = format!("{}{}", icon::BATTERY, icon::STRIKETHROUGH);
        return Ok(e);
    }
    e.full_text = {
        let mut split = e.full_text.split(' ');
        let status = split.next().unwrap();
        let percentage = split.next().unwrap();
        let icon = match status {
            "BAT" => icon::BATTERY,
            "CHR" => icon::HOURGLASS,
            "FULL" | "IDLE" => icon::CABLE,
            status => return Err(format!("Unknown Battery Status: {}", status))
        };
        let mut res = format!("{} {}", icon, percentage);
        if let Some(remaining) = split.next() {
            res += " ";
            res += &remaining;
        }
        res
    };
    Ok(e)
}

fn cpu_usage(mut e: Block) -> Block {
    e.full_text = format!("{} {}", icon::PC, e.full_text);
    e
}

fn load(mut e: Block) -> Block {
    let icon = match e.color.as_ref().map(String::as_ref) {
        Some("#FF0000") => icon::LIGHTNING,
        _ => icon::WARNING,
    };
    e.full_text = format!("{} {}", icon, e.full_text);
    e
}

