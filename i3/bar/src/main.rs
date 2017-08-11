extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_process;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate bytes;
extern crate futures;
extern crate chrono;

mod controller;
mod i3status;
mod icon;

use std::cell::RefCell;
use std::rc::Rc;

use tokio_core::reactor::Core;

use controller::Controller;

fn main() {
    let mut core = Core::new().unwrap();

    let controller = Rc::new(RefCell::new(Controller::new()));
    let i3status = i3status::i3status(controller, &core.handle());
    // TODO: brightness
    // TODO: media
    // TODO: moon phase
    // TODO: RAM
    // TODO: SWAP

    core.run(i3status).unwrap();
}
