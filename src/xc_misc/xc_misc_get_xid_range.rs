use ::std::io;

use ::futures::Future;
use ::byteorder::NativeEndian;
use ::byteorder::WriteBytesExt;
use ::byteorder::ReadBytesExt;
use ::tokio_io;

use ::protocol::ExtensionRequest;
use ::protocol::ExtensionInfo;
use ::xproto::Xid;
use ::Client;

const XC_MISC_OPCODE: u8 = 1;

#[derive(Debug, Clone, Copy)]
pub struct XCMiscGetXIDRange;

impl ExtensionRequest for XCMiscGetXIDRange {
    type Reply = XCMiscGetXIDRangeReply;

    fn extension_name() -> &'static [u8] {
        b"XC-MISC"
    }

    fn encode(&mut self, info: &ExtensionInfo) -> io::Result<Vec<u8>> {
        let mut a = io::Cursor::new(vec![]);

        a.write_u8(info.major_opcode)?;
        a.write_u8(XC_MISC_OPCODE)?;
        a.write_u16::<NativeEndian>(1)?;

        Ok(a.into_inner())
    }

    fn decode(client: Client) -> Box<Future<Item = (Client, Self::Reply), Error = io::Error>> {
        let buf: [u8; 32] = [0u8; 32];
        Box::new(tokio_io::io::read_exact(client, buf).and_then(|(client, buf)| {
            let mut a = io::Cursor::new(buf);

            a.read_u8()?;
            a.read_u8()?;
            a.read_u16::<NativeEndian>()?;
            a.read_u32::<NativeEndian>()?;
            let start_id = a.read_u32::<NativeEndian>()?;
            let count = a.read_u32::<NativeEndian>()?;

            let reply = XCMiscGetXIDRangeReply {
                start_id: start_id,
                count: count,
            };

            Ok((client, reply))
        }))
    }
}

/// Reply of `QueryExtension` request.
#[derive(Debug, Clone, Copy)]
pub struct XCMiscGetXIDRangeReply {
    /// The first ID in the range.
    pub start_id: Xid,

    /// The number of IDs in the range.
    pub count: Xid,
}
