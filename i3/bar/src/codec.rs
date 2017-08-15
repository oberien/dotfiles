use std::io;

use serde_json as json;
use tokio_io::codec::Encoder as Enc;
use bytes::BytesMut;

// TODO: Proper color (de-)serialization

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Element {
    Header(Header),
    OpenStream,
    Blocks(Vec<Block>),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Header {
    pub version: u32,
    pub stop_signal: Option<u32>,
    pub cont_signal: Option<u32>,
    pub click_events: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Align {
    #[serde(rename = "center")] Center,
    #[serde(rename = "right")] Right,
    #[serde(rename = "left")] Left,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Markup {
    #[serde(rename = "none")] None,
    #[serde(rename = "pango")] Pango,
}

impl Default for Markup {
    fn default() -> Markup {
        Markup::None
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct ClickEvent {
    pub name: Option<String>,
    pub instance: Option<String>,
    pub x: u32,
    pub y: u32,
    pub button: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Block {
    pub full_text: String,
    pub short_text: Option<String>,
    pub color: Option<String>,
    pub background: Option<String>,
    pub border: Option<String>,
    pub min_width: Option<u32>,
    pub align: Option<Align>,
    pub name: Option<String>,
    pub instance: Option<String>,
    pub urgent: Option<bool>,
    pub separator: Option<bool>,
    pub separator_block_width: Option<u32>,
    pub markup: Markup,
}

enum State {
    // {"version":1}
    AwaitingHeader,
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
            state: State::AwaitingHeader,
        }
    }

    pub fn decode_line(&mut self, line: &str) -> io::Result<Element> {
        match self.state {
            State::AwaitingHeader => {
                let res = json::from_str(line)?;
                self.state = State::AwaitingOpenStream;
                Ok(Element::Header(res))
            }
            State::AwaitingOpenStream => {
                assert_eq!(line, "[");
                self.state = State::AwaitingData;
                Ok(Element::OpenStream)
            }
            State::AwaitingData => Ok(json::from_str(line)?)
        }
    }
}

pub struct Encoder;

impl Enc for Encoder {
    type Item = Element;
    type Error = io::Error;

    fn encode(&mut self, element: Element, dst: &mut BytesMut) -> Result<(), io::Error> {
        let line = match element {
            Element::Header(data) => json::to_string(&data)? + "\n",
            Element::Blocks(data) => json::to_string(&data)? + ",\n",
            Element::OpenStream => "[\n".to_string(),
        };
        dst.extend(line.bytes());
        Ok(())
    }
}

pub fn decode_event(line: String) -> io::Result<ClickEvent> {
    Ok(json::from_str(&line)?)
}

#[derive(Debug, Clone)]
pub struct BlockBuilder {
    block: Block,
}

impl BlockBuilder {
    pub fn new(full_text: String) -> BlockBuilder {
        BlockBuilder {
            block: Block {
                full_text,
                ..Block::default()
            }
        }
    }
    #[allow(dead_code)]
    pub fn short_text(mut self, short_text: String) -> BlockBuilder {
        self.block.short_text = Some(short_text);
        self
    }
    #[allow(dead_code)]
    pub fn color(mut self, color: String) -> BlockBuilder {
        self.block.color = Some(color);
        self
    }
    #[allow(dead_code)]
    pub fn background(mut self, background: String) -> BlockBuilder {
        self.block.background = Some(background);
        self
    }
    #[allow(dead_code)]
    pub fn border(mut self, border: String) -> BlockBuilder {
        self.block.border = Some(border);
        self
    }
    #[allow(dead_code)]
    pub fn min_width(mut self, min_width: u32) -> BlockBuilder {
        self.block.min_width = Some(min_width);
        self
    }
    #[allow(dead_code)]
    pub fn align(mut self, align: Align) -> BlockBuilder {
        self.block.align = Some(align);
        self
    }
    #[allow(dead_code)]
    pub fn name(mut self, name: String) -> BlockBuilder {
        self.block.name = Some(name);
        self
    }
    #[allow(dead_code)]
    pub fn instance(mut self, instance: String) -> BlockBuilder {
        self.block.instance = Some(instance);
        self
    }
    #[allow(dead_code)]
    pub fn urgent(mut self, urgent: bool) -> BlockBuilder {
        self.block.urgent = Some(urgent);
        self
    }
    #[allow(dead_code)]
    pub fn separator(mut self, separator: bool) -> BlockBuilder {
        self.block.separator = Some(separator);
        self
    }
    #[allow(dead_code)]
    pub fn separator_block_width(mut self, separator_block_width: u32) -> BlockBuilder {
        self.block.separator_block_width = Some(separator_block_width);
        self
    }
    #[allow(dead_code)]
    pub fn markup(mut self, markup: Markup) -> BlockBuilder {
        self.block.markup = markup;
        self
    }
    pub fn build(self) -> Block {
        self.block
    }
}
