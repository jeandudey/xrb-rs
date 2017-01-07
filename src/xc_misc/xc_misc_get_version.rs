use ::std::io;

use ::futures::Future;
use ::byteorder::NativeEndian;
use ::byteorder::WriteBytesExt;
use ::byteorder::ReadBytesExt;
use ::tokio_core;

use ::protocol::ExtensionRequest;
use ::protocol::ExtensionInfo;
use ::Client;

const XC_MISC_OPCODE: u8 = 0;

#[derive(Debug, Clone, Copy)]
pub struct XCMiscGetVersion {
    /// Indicates what version of the protocol the client wants the server to implement.
    pub client_major_version: u16,

    /// Indicates what version of the protocol the client wants the server to implement.
    pub client_minor_version: u16,
}

impl ExtensionRequest for XCMiscGetVersion {
    type Reply = XCMiscGetVersionReply;

    fn extension_name() -> &'static [u8] {
        b"XC-MISC"
    }

    fn encode(&mut self, info: &ExtensionInfo) -> io::Result<Vec<u8>> {
        let mut a = io::Cursor::new(vec![]);

        a.write_u8(info.major_opcode)?;
        a.write_u8(XC_MISC_OPCODE)?;
        a.write_u16::<NativeEndian>(2)?;
        a.write_u16::<NativeEndian>(self.client_major_version)?;
        a.write_u16::<NativeEndian>(self.client_minor_version)?;

        Ok(a.into_inner())
    }

    fn decode(client: Client) -> Box<Future<Item = (Client, Self::Reply), Error = io::Error>> {
        let buf: [u8; 32] = [0u8; 32];
        Box::new(tokio_core::io::read_exact(client, buf).and_then(|(client, buf)| {
            let mut a = io::Cursor::new(buf);

            a.read_u8()?;
            a.read_u8()?;
            a.read_u16::<NativeEndian>()?;
            a.read_u32::<NativeEndian>()?;
            let major = a.read_u16::<NativeEndian>()?;
            println!("got here!5");
            let minor = a.read_u16::<NativeEndian>()?;
            println!("got here!6");

            let reply = XCMiscGetVersionReply {
                server_major_version: major,
                server_minor_version: minor,
            };

            Ok((client, reply))
        }))
    }
}

/// Reply of `QueryExtension` request.
#[derive(Debug, Clone, Copy)]
pub struct XCMiscGetVersionReply {
    /// Indicates the version that server actually supports.
    pub server_major_version: u16,

    /// Indicates the version that server actually supports.
    pub server_minor_version: u16,
}
