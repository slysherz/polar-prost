extern crate prost;

pub use prost::{Message, EncodeError, DecodeError};
pub mod data;
pub mod types;
pub mod protocol;

pub fn encode<M: Message>(value: M) -> Result<Vec<u8>, prost::EncodeError> {
    let mut buffer = vec![0u8; 0];
    value.encode(&mut buffer)?;

    Ok(buffer)
}