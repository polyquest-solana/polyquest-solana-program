use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use std::io;
use wormhole_anchor_sdk::token_bridge;
use wormhole_io::Readable;

const PAYLOAD_ID_MESSAGE: u8 = 1;

#[derive(Clone, Copy)]
pub enum PolyQuestTokenMessage {
    Message { recipient: [u8; 32] },
}

impl AnchorSerialize for PolyQuestTokenMessage {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            PolyQuestTokenMessage::Message { recipient } => {
                PAYLOAD_ID_MESSAGE.serialize(writer)?;
                recipient.serialize(writer)
            }
        }
    }
}

impl AnchorDeserialize for PolyQuestTokenMessage {
    fn deserialize_reader<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        match u8::read(reader)? {
            PAYLOAD_ID_MESSAGE => Ok(PolyQuestTokenMessage::Message {
                recipient: Readable::read(reader)?,
            }),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid payload ID",
            )),
        }
    }
}

pub type PostedPolyQuestTokenMessage = token_bridge::PostedTransferWith<PolyQuestTokenMessage>;
