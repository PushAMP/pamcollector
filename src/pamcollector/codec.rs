use std::io;
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder};
use pamcollector::metric::Metric;
use serde_cbor;

pub struct LineCodec;

impl Decoder for LineCodec {
    type Item = Metric;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Metric>> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.split_to(i);
            // Also remove the '\n'
            buf.split_to(1);
            let metric: Result<Metric, _> = serde_cbor::from_slice(&line);
            match metric {
                Ok(me) => Ok(Some(me)),
                Err(_) => {
                    Err(io::Error::new(io::ErrorKind::Other,
                                       "Invalid input, unable to parse as a JSON object"))
                }
            }
        } else {
            Ok(None)
        }
    }
}


impl Encoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(msg.as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}

