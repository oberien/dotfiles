use std::rc::Rc;
use std::cell::RefCell;
use std::io;
use std::time::Instant;

use tokio_core::reactor::Handle;
use futures::{future, Future, Stream};

use sysfs::Sysfs;
use controller::Controller;
use codec::BlockBuilder;
use icon;

#[derive(Debug)]
enum Which {
    Full(u32),
    Now(u32),
    State(String),
}

pub fn battery(controller: Rc<RefCell<Controller>>, handle: &Handle) -> Box<Future<Item=(), Error=io::Error>> {
    let path = match Sysfs::batteries() {
        Ok(ref mut vec) if vec.len() > 0 => vec.remove(0),
        _ => return Box::new(future::ok(()))
    };
    let full = Sysfs::new(path.join("charge_full_design"), handle).unwrap()
        .and_then(|s| s.trim().parse::<u32>().map_err(|e| io::Error::new(io::ErrorKind::Other, e)))
        .map(|full| Which::Full(full));
    let now = Sysfs::new(path.join("charge_now"), handle).unwrap()
        .and_then(|s| s.trim().parse::<u32>().map_err(|e| io::Error::new(io::ErrorKind::Other, e)))
        .map(|now| Which::Now(now));
    let state = Sysfs::new(path.join("status"), handle).unwrap()
        .map(|state| Which::State(state));
    let selected = full.select(now).select(state);
    let mut full = None;
    let mut now = None;
    let mut state = None;
    let mut ts: Option<Instant> = None;
    let mut charge = None;
    let mut avg = 0.0;
    let future = selected.for_each(move |item| {
        eprintln!("item: {:?}", item);
        match item {
            Which::Full(f) => full = Some(f),
            Which::Now(n) => now = Some(n),
            Which::State(s) => {
                // if state changed, reset moving average values
                if state.is_none() || state.as_ref().unwrap() != &s {
                    ts = None;
                    charge = None;
                    avg = 0.0;
                }
                state = Some(s)
            },
        }
        match (full, now, &state) {
            (Some(full), Some(now), &Some(ref state)) => {
                let mut controller = controller.borrow_mut();
                // icon
                let icon = match state.trim() {
                    "Charging" => icon::HOURGLASS,
                    "Discharging" => icon::BATTERY,
                    "Full" | "Idle" => icon::CABLE,
                    unk => {
                        controller.push_error(format!("Unknown Battery Status: {}", unk));
                        return Ok(())
                    }
                };

                // percentage
                let percentage = now as f32 / full as f32 * 100.0;

                // moving average
                if state.trim() == "Charging" || state.trim() == "Discharging" {
                    let instant = Instant::now();
                    if ts.is_some() && charge.is_some() {
                        let dt = instant - ts.unwrap();
                        let dc = now - charge.unwrap();
                        let dur = now as f64 / dc as f64 * dt.as_secs() as f64;
                        let exp = 600.0;
                        avg = (avg * exp + dur * (2048.0 - exp)) / 2048.0;
                    }
                    ts = Some(instant);
                    charge = Some(now);
                }
                let mut text = format!("{} {:.2}%", icon, percentage);
                if avg != 0.0 {
                    let seconds = avg as u32 % 60;
                    let minutes = (avg / 60.0) as u32 % 60;
                    let hours = (avg / 3600.0) as u8;
                    text += &format!(" {}:{}:{}", hours, minutes, seconds);
                }
                controller.set_battery(BlockBuilder::new(text)
                    .name("battery".to_string())
                    .instance(path.to_string_lossy().to_string())
                    .build());
                controller.update();
            }
            _ => ()
        }
        Ok(())
    });
    Box::new(future)
}
