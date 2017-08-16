use std::rc::Rc;
use std::cell::RefCell;
use std::io;

use tokio_core::reactor::Handle;
use futures::{future, Future, Stream};
use futures::stream::MergedItem;

use sysfs::Sysfs;
use controller::Controller;
use codec::BlockBuilder;
use icon;

pub fn backlight(controller: Rc<RefCell<Controller>>, handle: &Handle) -> Box<Future<Item=(), Error=io::Error>> {
    let max = Sysfs::new_absolute("/sys/class/backlight/intel_backlight/max_brightness", handle);
    let actual = Sysfs::new_absolute("/sys/class/backlight/intel_backlight/actual_brightness", handle);
    let (max, actual) = match (max, actual) {
        (Ok(max), Ok(actual)) => (max, actual),
        // ignore nonexistent backlight
        _ => return Box::new(future::ok(()))
    };
    let max = max.and_then(|s| s.trim().parse::<u32>().map_err(|e| io::Error::new(io::ErrorKind::Other, e)));
    let actual = actual.and_then(|s| s.trim().parse::<u32>().map_err(|e| io::Error::new(io::ErrorKind::Other, e)));
    let merged = max.merge(actual);
    let mut max = None;
    let mut actual = None;
    let future = merged.for_each(move |item| {
        match item {
            MergedItem::First(val) => max = Some(val),
            MergedItem::Second(val) => actual = Some(val),
            MergedItem::Both(m, a) => {
                max = Some(m);
                actual = Some(a);
            }
        }
        if max.is_some() && actual.is_some() {
            let (max, actual) = (max.unwrap(), actual.unwrap());
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
        }
        Ok(())
    });
    Box::new(future)
}
