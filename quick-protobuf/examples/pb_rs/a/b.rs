use super::super::*;
use quick_protobuf::sizeofs::*;
use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Result, Writer};
use std::io::Write;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ImportedMessage {
    pub i: Option<bool>,
}

impl<'a> MessageRead<'a> for ImportedMessage {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.i = Some(r.read_bool(bytes)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ImportedMessage {
    fn get_size(&self) -> usize {
        0 + self
            .i
            .as_ref()
            .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.i {
            w.write_with_tag(8, |w| w.write_bool(*s))?;
        }
        Ok(())
    }
}
