use std::io;

use tokio_io::codec::{Encoder, Decoder};
use serde_json as json;
use bytes::BytesMut;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Element {
    pub name: String,
    pub instance: Option<String>,
    pub markup: String,
    pub full_text: String,
    pub color: Option<String>,
}

enum State {
    // {"version":1}
    AwaitingVersion,
    // [
    AwaitingOpenStream,
    // [{...},...]
    AwaitingData,
}

pub struct Codec {
    state: State,
}

impl Codec {
    pub fn new() -> Codec {
        Codec {
            state: State::AwaitingVersion,
        }
    }

    pub fn decode_line(&mut self, line: &str) -> io::Result<Option<Vec<Element>>> {
        match self.state {
            State::AwaitingVersion => {
                assert_eq!(line, "{\"version\":1}");
                self.state = State::AwaitingOpenStream;
                Ok(None)
            }
            State::AwaitingOpenStream => {
                assert_eq!(line, "[");
                self.state = State::AwaitingData;
                Ok(None)
            }
            State::AwaitingData => json::from_str(line).map_err(|e| e.into()).map(Some)
        }
    }
}

impl Encoder for Codec {
    type Item = Vec<Element>;
    type Error = io::Error;

    fn encode(&mut self, item: Vec<Element>, dst: &mut BytesMut) -> Result<(), io::Error> {
        let s = json::to_string(&item)?;
        dst.extend(s.bytes());
        Ok(())
    }
}

impl Decoder for Codec {
    type Item = Vec<Element>;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Vec<Element>>, io::Error> {
        if let Some(pos) = src.iter().position(|&b| b == b'\n') {
            let line = src.split_to(pos);
            // remove \n
            src.split_to(1);
            match json::from_slice(&line) {
                Ok(elements) => {
                    Ok(Some(elements))
                }
                Err(e) => Err(e.into())
            }
        } else {
            return Ok(None)
        }
    }
}
