use std::rc::Rc;
use std::cell::RefCell;
use std::io::{self, Read};
use std::fs::File;
use std::mem;

use tokio_core::reactor::{PollEvented, Handle};
use futures::{future, Future, Stream, Async, Poll};
use libc;

use my_io::MyIo;
use controller::Controller;
use codec::BlockBuilder;
use icon;

pub fn backlight(controller: Rc<RefCell<Controller>>, handle: &Handle) -> Box<Future<Item=(), Error=io::Error>> {
    let backlight = match Backlight::new(handle) {
        Ok(b) => b,
        // ignore no backlight
        Err(_) => return Box::new(future::ok(()))
    };
    let future = backlight.for_each(move |(actual, max)| {
        let mut controller = controller.borrow_mut();
        let icon = match actual < max / 2 {
            true => icon::LOW_BRIGHTNESS,
            false => icon::HIGH_BRIGHTNESS
        };
        controller.set_backlight(BlockBuilder::new(format!("{} {}", icon, actual))
            .name("brightness".to_string())
            .instance("/sys/class/backlight/intel_backlight/".to_string())
            .build());
        controller.update();
        Ok(())
    });
    Box::new(future)
}

pub struct Backlight {
    max_buf: String,
    actual_buf: String,
    max: PollEvented<MyIo>,
    actual: PollEvented<MyIo>,
    max_brightness: Option<u32>,
    actual_brightness: Option<u32>,
}

impl Backlight {
    pub fn new(handle: &Handle) -> io::Result<Backlight> {
        // FIXME: next level lul
        let max = File::open("/sys/class/backlight/intel_backlight/max_brightness")?;
        let actual = File::open("/sys/class/backlight/intel_backlight/actual_brightness")?;
        let res = Backlight {
            max_buf: String::new(),
            actual_buf: String::new(),
            max: PollEvented::new(MyIo::new(&max), handle)?,
            actual: PollEvented::new(MyIo::new(&actual), handle)?,
            max_brightness: None,
            actual_brightness: None,
        };
        mem::forget(max);
        mem::forget(actual);
        Ok(res)
    }
}

impl Stream for Backlight {
    type Item = (u32, u32);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.max.need_read();
        self.actual.need_read();
        let (pe, buf, max) = if self.max.poll_read() == Async::Ready(()) {
            (&mut self.max, &mut self.max_buf, true)
        } else if self.actual.poll_read() == Async::Ready(()) {
            (&mut self.actual, &mut self.actual_buf, false)
        } else {
            return Ok(Async::NotReady)
        };
        match pe.read_to_string(buf) {
            Ok(_) => (),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                return Ok(Async::NotReady);
            }
            Err(e) => return Err(e)
        }
        assert_eq!(buf.pop(), Some('\n'));
        let value = buf.parse().unwrap();
        buf.clear();
        let res = unsafe { libc::lseek(pe.get_ref().fd, 0, libc::SEEK_SET) };
        assert!(res >= 0);
        if max {
            self.max_brightness = Some(value);
        } else {
            self.actual_brightness = Some(value);
        }
        if self.max_brightness.is_none() || self.actual_brightness.is_none() {
            return Ok(Async::NotReady);
        }
        Ok(Async::Ready(Some((self.actual_brightness.unwrap(), self.max_brightness.unwrap()))))
    }
}