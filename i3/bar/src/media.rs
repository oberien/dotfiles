use std::io;
use std::net::ToSocketAddrs;
use std::rc::Rc;
use std::cell::RefCell;

use futures::{Future, Poll, Async};
use mpd::Client;
use mpd::error::{Result, Error};
use tokio_core::reactor::Handle;
use tokio_core::net::TcpStream;

use controller::Controller;

pub fn mpd(controller: Rc<RefCell<Controller>>, handle: &Handle) -> Box<Future<Item=(), Error=io::Error>> {
    let socket = TcpStream::connect(&("localhost", 6600).to_socket_addrs().unwrap().next().unwrap(), handle);
    let mpd = socket.and_then(|s| tr(s, |s| Client::new(s.try_clone().unwrap())));
    Box::new(mpd.and_then(|c| tr(c, |c| c.status()))
        .map(|s| println!("{:?}", s)))
}

struct Transform<T, U, F: FnMut(&U) -> Result<T>>(U, F);

impl<T, U, F: FnMut(&U) -> Result<T>> Future for Transform<T, U, F> {
    type Item = T;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<T, io::Error> {
        let res = match self.1(self.0) {
            Ok(t) => Ok(t),
            Err(e) => match e {
                Error::Io(io) => Err(io),
                e => Err(io::Error::new(io::ErrorKind::Other, e)),
            }
        };
        Ok(Async::Ready(try_nb!(res)))
    }
}

// transform result
fn tr<T, U, F: FnMut(&U) -> Result<T>>(u: U, fun: F) -> Transform<T, U, F> {
    Transform(u, fun)
//    let res = try_nb!(fun());
//    match res {
//        Ok(t) => Ok(t),
//        Err(e) => match e {
//            Error::Io(io) => Err(io),
//            e => Err(io::Error::new(io::ErrorKind::Other, e)),
//            Error::Parse(parse) => io::Error::new(io::ErrorKind::Other, parse),
//            Error::Proto(proto) =>
//            Error::Server(server)
//        }
//    }
}
