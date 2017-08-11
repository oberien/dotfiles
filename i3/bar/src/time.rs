use std::io;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::{Instant, Duration};

use futures::{Future, Stream};
use tokio_timer::Timer;
use chrono::{Utc, Timelike};

use controller::Controller;
use i3status::codec::Element;
use icon;

pub fn time(controller: Rc<RefCell<Controller>>) -> Box<Future<Item=(), Error=io::Error>> {
    let timer = Timer::default().interval_at(Instant::now(), Duration::new(1, 0));
    let future = timer.for_each(move |()| {
        let mut controller = controller.borrow_mut();
        let datetime = Utc::now();
        let offset = (datetime.hour() % 12) * 2 + (datetime.minute() + 15) / 30;
        let clock = icon::CLOCKS[offset as usize];
        let offset = (datetime.hour() + 2) / 6;
        let sun = icon::CYCLE[offset as usize];
        let time = datetime.format("%T");
        let time = Element {
            name: "time".to_string(),
            instance: None,
            markup: "none".to_string(),
            full_text: format!("{} {} {}", clock, time, sun),
            color: None,
        };
        controller.set_time(time);
        let date = datetime.format("%F");
        let date = Element {
            name: "date".to_string(),
            instance: None,
            markup: "none".to_string(),
            full_text: format!("{} {}", icon::CALENDAR, date),
            color: None,
        };
        controller.set_date(date);
        controller.update();
        Ok(())
    });
    Box::new(future.map_err(|e| e.into()))
}
