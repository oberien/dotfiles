use std::io::{self, Read, Seek, SeekFrom};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
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
    pub fn new<P: AsRef<Path>>(path: P, handle: &Handle) -> io::Result<Sysfs> {
        let file = File::open(path)?;
        Ok(Sysfs {
            buf: String::new(),
            pe: AsyncFile::raw_new(file).into_io(handle)?,
        })
    }

    pub fn backlights() -> io::Result<Vec<PathBuf>> {
        Self::read_dir("/sys/class/backlight/", None)
    }

    pub fn batteries() -> io::Result<Vec<PathBuf>> {
        Self::read_dir("/sys/class/power_supply/", Some("BAT"))
    }

    pub fn read_dir<P: AsRef<Path>>(dir: P, prefix: Option<&str>) -> io::Result<Vec<PathBuf>> {
        let iter = fs::read_dir(dir)?;
        let res = iter.filter_map(|f| {
            if let Ok(ref entry) = f {
                if let Ok(typ) = entry.file_type() {
                    let path = entry.path();
                    let name = path.file_name().unwrap().to_str().unwrap();
                    if prefix.is_none() || name.starts_with(prefix.unwrap()) {
                        return Some(entry.path());
                    }
                }
            }
            None
        }).collect();
        Ok(res)
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
