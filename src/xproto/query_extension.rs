use ::std::io;
use ::std::io::Write;

use ::futures::Future;
use ::byteorder::NativeEndian;
use ::byteorder::WriteBytesExt;
use ::byteorder::ReadBytesExt;
use ::tokio_core;

use ::protocol::Request;
use ::Client;
use ::pad;

const OPCODE: u8 = 98;

#[derive(Debug)]
pub struct QueryExtension {
    /// The extension to query.
    pub name: Vec<u8>,
}

impl Request for QueryExtension {
    type Reply = QueryExtensionReply;

    fn encode(&mut self) -> io::Result<Vec<u8>> {
        let mut a = io::Cursor::new(vec![]);

        let n = self.name.len();
        let p = pad(n);
        let len = 2 + ((n + p) / 4);

        a.write_u8(OPCODE)?;
        a.write_u8(0)?;

        a.write_u16::<NativeEndian>(len as u16)?;
        a.write_u16::<NativeEndian>(n as u16)?;
        a.write_u16::<NativeEndian>(0)?;

        a.write(self.name.as_slice())?;

        for _ in 0..p {
            a.write_u8(0)?;
        }

        Ok(a.into_inner())
    }

    fn decode(client: Client) -> Box<Future<Item = (Client, Self::Reply), Error = io::Error>> {
        let buf: [u8; 32] = [0u8; 32];
        tokio_core::io::read_exact(client, buf)
            .and_then(|(client, buf)| {
                let mut a = io::Cursor::new(buf);

                a.read_u8()?;
                a.read_u8()?;
                a.read_u16::<NativeEndian>()?;
                a.read_u32::<NativeEndian>()?;
                let present = a.read_u8()? == 1;
                let major_opcode = a.read_u8()?;
                let first_event = a.read_u8()?;
                let first_error = a.read_u8()?;

                Ok((client,
                    QueryExtensionReply {
                    present: present,
                    major_opcode: major_opcode,
                    first_event: first_event,
                    first_error: first_error,
                }))
            })
            .boxed()
    }
}

/// Reply of `QueryExtension` request.
#[derive(Debug, Clone, Copy)]
pub struct QueryExtensionReply {
    /// Determines if the extension is present.
    pub present: bool,

    /// The major opcode of the extension, if it has one. Otherwise,
    /// zero is returned.
    pub major_opcode: u8,

    /// If the extension involves additional event types, the base event
    /// code is returned. Otherwise, zero is returned.
    pub first_event: u8,

    /// If the extension involves additional event types, the base error
    /// code is returned. Otherwise, zero is returned.
    pub first_error: u8,
}
