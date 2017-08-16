use std::io::{self, Read, Seek, SeekFrom};
use std::fs::File;
use std::path::Path;
use std::os::unix::io::{FromRawFd, AsRawFd};
use std::mem;

use tokio_core::reactor::{PollEvented, Handle};
use futures::{Stream, Async, Poll};
use tokio_file_unix::File as AsyncFile;

pub struct Sysfs {
    buf: String,
    pe: PollEvented<AsyncFile<File>>,
}

impl Sysfs {
    pub fn new_absolute<P: AsRef<Path>>(path: P, handle: &Handle) -> io::Result<Sysfs> {
        let file = File::open(path)?;
        Ok(Sysfs {
            buf: String::new(),
            pe: AsyncFile::raw_new(file).into_io(handle)?,
        })
    }
}

impl Stream for Sysfs {
    type Item = String;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.pe.need_read();
        if self.pe.poll_read() == Async::NotReady {
            return Ok(Async::NotReady);
        }
        match self.pe.read_to_string(&mut self.buf) {
            Ok(_) => (),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => return Ok(Async::NotReady),
            Err(e) => return Err(e)
        }

        // we need to seek to 0 to get notified again
        let mut file = unsafe { File::from_raw_fd(self.pe.get_ref().as_raw_fd()) };
        file.seek(SeekFrom::Start(0))?;
        mem::forget(file);

        let res = self.buf.clone();
        self.buf.clear();
        Ok(Async::Ready(Some(res)))
    }
}
