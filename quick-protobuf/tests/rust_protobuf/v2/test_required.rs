use super::*;
use quick_protobuf::sizeofs::*;
use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Result, Writer};
use std::io::Write;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestRequired {
    pub b: bool,
}

impl<'a> MessageRead<'a> for TestRequired {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(40) => msg.b = r.read_bool(bytes)?,
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestRequired {
    fn get_size(&self) -> usize {
        0 + 1 + sizeof_varint(*(&self.b) as u64)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(40, |w| w.write_bool(*&self.b))?;
        Ok(())
    }
}
