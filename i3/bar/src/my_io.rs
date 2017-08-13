use std::os::unix::io::{RawFd, AsRawFd};
use std::io::{self, Read};

use mio::{Ready, Poll, PollOpt, Token};
use mio::event::Evented;
use mio::unix::EventedFd;
use libc;

// ripped straight from the mio docs
// need this because EventedFd only takes a reference
// seriously!?
pub struct MyIo {
    pub fd: RawFd,
}

impl MyIo {
    pub fn new<F: AsRawFd>(f: &F) -> MyIo {
        let fd = f.as_raw_fd();
        MyIo {
            fd
        }
    }
}

impl Read for MyIo {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let ptr = buf.as_mut_ptr() as *mut libc::c_void;
        let res = unsafe { libc::read(self.fd, ptr, buf.len()) };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(res as usize)
    }
}

impl Evented for MyIo {
    fn register(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt)
                -> io::Result<()>
    {
        EventedFd(&self.fd).register(poll, token, interest, opts)
    }

    fn reregister(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt)
                  -> io::Result<()>
    {
        EventedFd(&self.fd).reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        EventedFd(&self.fd).deregister(poll)
    }
}
