use ::std::io;

use ::futures::Future;
use ::byteorder::NativeEndian;
use ::byteorder::WriteBytesExt;
use ::byteorder::ReadBytesExt;
use ::tokio_core;

use ::protocol::ExtensionRequest;
use ::protocol::ExtensionInfo;
use ::Client;

const XC_MISC_OPCODE: u8 = 2;

#[derive(Debug, Clone, Copy)]
pub struct XCMiscGetXIDList {
    /// The number of IDs to be requested.
    pub count: u32,
}

impl ExtensionRequest for XCMiscGetXIDList {
    type Reply = Vec<u32>; // TODO: Reemplaze u32 with Xid.

    fn extension_name() -> &'static [u8] {
        b"XC-MISC"
    }

    fn encode(&mut self, info: &ExtensionInfo) -> io::Result<Vec<u8>> {
        let mut a = io::Cursor::new(vec![]);

        a.write_u8(info.major_opcode)?;
        a.write_u8(XC_MISC_OPCODE)?;
        a.write_u16::<NativeEndian>(2)?;
        a.write_u32::<NativeEndian>(self.count)?;

        Ok(a.into_inner())
    }

    #[cfg_attr(feature = "dev", allow(needless_range_loop))]
    fn decode(client: Client) -> Box<Future<Item = (Client, Self::Reply), Error = io::Error>> {
        let buf: [u8; 32] = [0u8; 32];
        Box::new(tokio_core::io::read_exact(client, buf)
            .and_then(|(client, buf)| {
                let mut a = io::Cursor::new(buf);

                a.read_u8()?;
                a.read_u8()?;
                a.read_u16::<NativeEndian>()?;
                a.read_u32::<NativeEndian>()?;
                let count = a.read_u32::<NativeEndian>()?;

                Ok((client, count as usize))
            })
            .and_then(|(client, count)| {
                let buf = vec![0u8; count * 4];
                tokio_core::io::read_exact(client, buf)
                    .map(move |(client, buf)| (client, count, buf))
            })
            .and_then(|(client, count, buf)| {
                let mut reply = vec![0u32; count];
                let mut a = io::Cursor::new(buf);

                for i in 0..count {
                    reply[i] = a.read_u32::<NativeEndian>()?;
                }

                Ok((client, reply))
            }))
    }
}
