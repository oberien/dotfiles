use std::io;

use serde_json as json;

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
