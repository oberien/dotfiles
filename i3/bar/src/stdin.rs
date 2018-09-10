use std::rc::Rc;
use std::cell::RefCell;
use std::io::{self, Read, BufReader};

use futures::{Future, Stream};
use tokio_core::reactor::{Handle, PollEvented};
use tokio_file_unix::{StdFile, File};
use tokio_io::AsyncRead;
use tokio_io::io as tokio_io;
use owning_ref::OwningHandle;

use codec;
use controller::Controller;

type OwnedStdout<'a> = OwningHandle<Box<io::Stdin>, Box<PollEvented<File<StdFile<io::StdinLock<'a>>>>>>;

struct StdRead<'a>(OwnedStdout<'a>);

impl<'a> Read for StdRead<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl<'a> AsyncRead for StdRead<'a> { }

pub fn stdin(controller: Rc<RefCell<Controller>>, handle: &Handle) -> Box<Future<Item=(), Error=io::Error>> {
    let owning = OwningHandle::new_with_fn(Box::new(io::stdin()), |stdin| {
        let stdin = unsafe { &*stdin as &io::Stdin };
        let lock = stdin.lock();
        let file = File::new_nb(StdFile(lock)).unwrap();
        let evented = file.into_io(handle).unwrap();
        Box::new(evented)
    });
    let stdin = StdRead(owning);
    let buffered = BufReader::new(stdin);
    let lines = tokio_io::lines(buffered);
    let stream = lines.and_then(codec::decode_event);
    let stream = stream.for_each(move |evt| {
        let mut controller = controller.borrow_mut();
        if let Some(name) = evt.as_ref().and_then(|e| e.name.as_ref()) {
            if name.starts_with("error") {
                controller.clear_error(&name);
            }
            if name == "mpd" {
                match evt.as_ref().unwrap().button {
                    // scroll up
                    4 => controller.mpd_next(),
                    // scroll down
                    5 => controller.mpd_prev(),
                    _ => controller.mpd_toggle()
                }
            }
        }
        Ok(())
    });
    Box::new(stream)
}
