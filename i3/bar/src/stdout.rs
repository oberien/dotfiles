use std::io;

use futures::{Future, Stream, Sink, Poll, StartSend};
use futures::sync::mpsc::{self, UnboundedSender};
use tokio_core::reactor::{Handle, PollEvented};
use tokio_file_unix::{StdFile, File};
use tokio_io::codec::FramedWrite;
use owning_ref::OwningHandle;

use codec::{Element, Encoder};

type OwnedStdout<'a> = OwningHandle<Box<io::Stdout>, Box<FramedWrite<PollEvented<File<StdFile<io::StdoutLock<'a>>>>, Encoder>>>;

struct StdSink<'a>(OwnedStdout<'a>);

impl<'a> Sink for StdSink<'a> {
    type SinkItem = Element;
    type SinkError = io::Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.0.start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.0.poll_complete()
    }

    fn close(&mut self) -> Poll<(), Self::SinkError> {
        self.0.close()
    }
}

pub fn stdout(handle: &Handle) -> UnboundedSender<Element> {
    let (send, recv) = mpsc::unbounded();
    let owning = OwningHandle::new_with_fn(Box::new(io::stdout()), |stdout| {
        let stdout = unsafe { &*stdout as &io::Stdout };
        let lock = stdout.lock();
        let file = File::new_nb(StdFile(lock)).unwrap();
        let evented = file.into_io(handle).unwrap();
        let framed = FramedWrite::new(evented, Encoder);
        Box::new(framed)
    });
    let stdout = StdSink(owning);
    handle.spawn(recv.forward(stdout.sink_map_err(|_| ())).map(|_| ()));
    send
}
