extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_process;
extern crate tokio_timer;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate bytes;
extern crate futures;
extern crate chrono;
extern crate sysinfo;

mod codec;
mod controller;
mod i3status;
mod systeminfo;
mod time;
mod icon;

use std::cell::RefCell;
use std::rc::Rc;

use tokio_core::reactor::Core;
use futures::future;

use controller::Controller;

fn main() {
    // TODO: Use tokio_file_unix for stdout handling
    // TODO: Make controller hold all streams and be a stream itself?
    // TODO: stdin with click event handling
    let mut core = Core::new().unwrap();

    let controller = Rc::new(RefCell::new(Controller::new()));
    let i3status = i3status::i3status(controller.clone(), &core.handle());
    let sysinfo = systeminfo::systeminfo(controller.clone());
    let time = time::time(controller.clone());
    // TODO: brightness
    // TODO: media
    // TODO: moon phase
    let joined = future::join_all(vec![i3status, sysinfo, time]);

    core.run(joined).unwrap();
}
