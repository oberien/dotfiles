use std::io;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::{Instant, Duration};

use futures::{Future, Stream};
use tokio_timer::Timer;
use sysinfo::{System, SystemExt};

use controller::Controller;
use codec::BlockBuilder;
use icon;

pub fn systeminfo(controller: Rc<RefCell<Controller>>) -> Box<Future<Item=(), Error=io::Error>> {
    let timer = Timer::default().interval_at(Instant::now(), Duration::new(5, 0));
    let system = Rc::new(RefCell::new(System::new()));
    let future = timer.for_each(move |()| {
        let mut controller = controller.borrow_mut();
        let mut system = system.borrow_mut();
        system.refresh_system();
        let used = convert_bytes(system.get_used_memory());
        let total = convert_bytes(system.get_total_memory());
        let ram = BlockBuilder::new(format!("{} {}/{}", icon::RAM, used, total))
            .name("ram".to_string())
            .instance("/proc/meminfo".to_string())
            .build();
        controller.set_ram(ram);
        let used = convert_bytes(system.get_used_swap());
        let total = convert_bytes(system.get_total_swap());
        let swap = BlockBuilder::new(format!("{} {}/{}", icon::FLOPPY, used, total))
            .name("swap".to_string())
            .instance("/proc/meminfo".to_string())
            .build();
        controller.set_swap(swap);
        controller.update();
        Ok(())
    });
    Box::new(future.map_err(|e| e.into()))
}

fn convert_bytes(bytes: u64) -> String {
    if bytes == 0 {
        return "0k".to_string();
    }
    let bytes = bytes as f64;
    // T should suffice to be future-proof
    let ending = ['k', 'M', 'G', 'T'];
    let exp = bytes.log(1024.0) as u32;
    let bytes = bytes / (1024u64.pow(exp)) as f64;
    format!("{:.1}{}", bytes, ending[exp as usize])
}