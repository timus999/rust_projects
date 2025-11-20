use bytes::{Buf, BytesMut};
use resp::{Decoder as RespDecoder, Value};
use std::io;
use std::io::Cursor;
use tokio_codec::{Decoder, Encoder};

pub struct RespCodec;

impl Encoder for RespCodec {
    type Item = Vec<u8>;
    type Error = io::Error;

    fn encode(&mut self, msg: Vec<u8>, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend_from_slice(&msg);
        Ok(())
    }
}

impl Decoder for RespCodec {
    type Item = Value;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Value>> {
        if buf.is_empty() {
            return Ok(None);
        }

        let mut reader = Cursor::new(buf.as_ref());
        match RespDecoder::new(&mut reader).decode() {
            Ok(value) => {
                let pos = reader.position() as usize;
                buf.advance(pos);
                Ok(Some(value))
            }
            Err(resp::Error::Incomplete) => Ok(None),
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
        }
    }
}
