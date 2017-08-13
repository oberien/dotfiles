use std::io;
use std::rc::Rc;
use std::cell::RefCell;
use std::thread;
use std::time::Duration;
use std::path::Path;

use futures::{Future, Stream};
use futures::sync::mpsc;
use mpd::{Client, State, Idle};
use mpd::idle::Subsystem;

use controller::Controller;
use icon;
use codec::{Block, BlockBuilder};
use timer::Fun;

#[derive(Debug)]
struct Info {
    file: Option<String>,
    title: Option<String>,
    artist: Option<String>,
    elapsed: Option<Duration>,
    total: Option<Duration>,
    state: State,
}

impl Info {
    fn to_element(&self) -> Block {
        let icon = match self.state {
            State::Play => icon::NOTE,
            State::Pause => icon::PAUSED,
            State::Stop => icon::STOPPED,
        };
        let full_text = if self.state == State::Stop {
            icon.to_string()
        } else {
            let elapsed = self.elapsed.unwrap().as_secs();
            let total = self.total.unwrap().as_secs();
            let mut name = String::new();
            if let Some(artist) = self.artist.as_ref() {
                name += &artist;
                name += " - ";
            }
            if let Some(title) = self.title.as_ref() {
                // some people just want to see the world burn
                // and put "Artist - Title" as title
                if title.contains(&name) {
                    name = String::new();
                }
                name += &title;
            } else {
                name += Path::new(self.file.as_ref().unwrap()).file_stem().unwrap().to_str().unwrap();
            }
            format!("{} ({}:{:02} / {}:{:02}) {}", icon, elapsed / 60, elapsed % 60, total / 60, total % 60, name)
        };
        BlockBuilder::new(full_text)
            .name("mpd".to_string())
            .build()
    }
}

pub fn mpd(controller: Rc<RefCell<Controller>>) -> (Box<Future<Item=(), Error=io::Error>>, Fun) {
    let (send, recv) = mpsc::unbounded();
    thread::spawn(move || {
        loop {
            if let Ok(mut mpd) = Client::connect(("127.0.0.1", 6600)) {
                loop {
                    let song = match mpd.currentsong() {
                        Ok(song) => song,
                        Err(_) => break
                    };
                    let status = match mpd.status() {
                        Ok(status) => status,
                        Err(_) => break
                    };
                    let mut file = None;
                    let mut title = None;
                    let mut artist = None;
                    if let Some(song) = song {
                        file = Some(song.file);
                        title = song.title;
                        artist = song.artist;
                    }
                    send.send(Some(Info {
                        file,
                        title,
                        artist,
                        elapsed: status.elapsed.map(|t| t.to_std().unwrap()),
                        total: status.time.map(|t| t.1.to_std().unwrap()),
                        state: status.state,
                    })).unwrap();
                    if mpd.wait(&[Subsystem::Player]).is_err() {
                        break;
                    }
                }
            }
            send.send(None).unwrap();
            // try to reconnect every 60 seconds
            thread::sleep(Duration::new(60, 0));
        }
    });
    let info = Rc::new(RefCell::new(None));
    let info2 = info.clone();
    let mpd = recv.for_each(move |opt| {
        controller.borrow_mut().set_media(opt.as_ref().map(|i| i.to_element()));
        controller.borrow_mut().update();
        *info.borrow_mut() = opt;
        Ok(())
    }).map_err(|_| unreachable!());

    let timer = move |mut controller: &mut Controller| {
        if let Some(ref mut info) = *info2.borrow_mut() {
            if info.state == State::Play {
                if let Some(ref mut time) = info.elapsed {
                    *time += Duration::new(1, 0);
                }
            }
            controller.set_media(Some(info.to_element()));
        }
        Ok(())
    };
    (Box::new(mpd), Box::new(timer))
}
