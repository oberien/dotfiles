use std::io;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::{Instant, Duration};

use futures::{Future, Stream};
use tokio_timer::Timer;

use controller::Controller;

pub type Fun = Box<FnMut(&mut Controller) -> io::Result<()>>;

pub fn execute(controller: Rc<RefCell<Controller>>, functions: Vec<Fun>) -> Box<Future<Item=(), Error=io::Error>> {
    let timer = Timer::default().interval_at(Instant::now(), Duration::new(1, 0))
        .map_err(|e| e.into());
    let functions = Rc::new(RefCell::new(functions));
    let future = timer.for_each(move |()| {
        let mut controller = controller.borrow_mut();
        for f in functions.borrow_mut().iter_mut() {
            f(&mut *controller)?;
        }
        controller.update();
        Ok(())
    });
    Box::new(future)
}
