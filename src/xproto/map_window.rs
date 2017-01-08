use ::std::io;

use ::byteorder::WriteBytesExt;
use ::byteorder::NativeEndian;
use ::futures;
use ::futures::Future;

use ::protocol::Request;
use ::protocol::VoidReply;
use ::xproto::Window;
use ::Client;

const OPCODE: u8 = 8;

pub struct MapWindow {
    /// The window to be mapped.
    pub wid: Window,
}

impl Request for MapWindow {
    type Reply = VoidReply;

    fn encode(&mut self) -> io::Result<Vec<u8>> {
        let mut a = io::Cursor::new(vec![]);
        let request_size: u16 = 2;

        a.write_u8(OPCODE)?;
        a.write_u8(0)?;
        a.write_u16::<NativeEndian>(request_size)?;
        a.write_u32::<NativeEndian>(self.wid)?;

        Ok(a.into_inner())
    }

    fn decode(client: Client) -> Box<Future<Item = (Client, Self::Reply), Error = io::Error>> {
        Box::new(futures::finished((client, ())))
    }
}
