use std::io;
use std::str;
use bytes::{BytesMut, BufMut};
use tokio_io::codec::{Encoder, Decoder};
use pamcollector::metric::Metric;
use serde_json;

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
            match str::from_utf8(&line) {
                Ok(s) => {
                    let met = serde_json::from_str(s);
                    match met {
                        Ok(metric) => Ok(Some(metric)),
                        Err(_) => {
                            Err(io::Error::new(io::ErrorKind::Other,
                                               "Invalid input, unable to parse as a JSON object"))
                        }
                    }
                }
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid UTF-8")),
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

