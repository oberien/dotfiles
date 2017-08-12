use std::io;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::{Instant, Duration};

use futures::{Future, Stream};
use tokio_timer::Timer;
use chrono::{Local, Timelike};

use controller::Controller;
use codec::BlockBuilder;
use icon;

pub fn time(controller: Rc<RefCell<Controller>>) -> Box<Future<Item=(), Error=io::Error>> {
    let timer = Timer::default().interval_at(Instant::now(), Duration::new(1, 0));
    let future = timer.for_each(move |()| {
        let mut controller = controller.borrow_mut();
        let datetime = Local::now();
        let offset = (datetime.hour() % 12) * 2 + (datetime.minute() + 15) / 30;
        let offset = offset % 24;
        let clock = icon::CLOCKS[offset as usize];
        let offset = (datetime.hour() + 2) / 6;
        let sun = icon::CYCLE[offset as usize];
        let time = datetime.format("%T");
        let time = BlockBuilder::new(format!("{} {} {}", clock, time, sun))
            .name("time".to_string())
            .build();
        controller.set_time(time);
        let date = datetime.format("%F");
        let date = BlockBuilder::new(format!("{} {}", icon::CALENDAR, date))
            .name("date".to_string())
            .build();
        controller.set_date(date);
        controller.update();
        Ok(())
    });
    Box::new(future.map_err(|e| e.into()))
}
