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
extern crate mpd;
extern crate tokio_file_unix;
extern crate owning_ref;

mod codec;
mod stdin;
mod stdout;
mod controller;
mod i3status;
mod systeminfo;
mod icon;
mod timer;
mod time;
mod media;
mod backlight;

use std::cell::RefCell;
use std::rc::Rc;

use tokio_core::reactor::Core;
use futures::future;

use controller::Controller;

fn main() {
    // TODO: Make controller hold all streams and be a stream itself?
    // TODO: Make mpd async
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let controller = Rc::new(RefCell::new(Controller::new(&handle)));
    let i3status = i3status::i3status(controller.clone(), &handle);
    let sysinfo = systeminfo::systeminfo(controller.clone());
    let (media, media_timer) = media::mpd(controller.clone());
    let timer = timer::execute(controller.clone(), vec![Box::new(time::time), media_timer]);
    let backlight = backlight::backlight(controller.clone(), &handle);
    let stdin = stdin::stdin(controller.clone(), &handle);
    // TODO: moon phase
    let joined = future::join_all(vec![i3status, sysinfo, timer, media, backlight, stdin]);

    core.run(joined).unwrap();
}
