use ::std::io;
use ::std::io::Write;

use ::futures;
use ::futures::Future;
use ::byteorder::NativeEndian;
use ::byteorder::WriteBytesExt;

use ::protocol::Request;
use ::protocol::VoidReply;
use ::xproto::Window;
use ::xproto::WindowAttributes;
use ::Client;

const OPCODE: u8 = 1;

pub struct CreateWindow {
    /// The window resource id.
    pub wid: Window,

    /// The window parent.
    pub parent: u32,

    /// The class
    pub class: u16,

    /// Window bit depth.
    pub depth: u8,

    /// Window visual
    pub visual: u32,

    /// Window x coordinate on parent.
    pub x: u16,

    /// Window y coordinate on parent.
    pub y: u16,

    /// Window width.
    pub width: u16,

    /// Window height.
    pub height: u16,

    /// The window border width.
    pub border_width: u16,

    pub attrs: WindowAttributes,
}

impl Request for CreateWindow {
    type Reply = VoidReply;

    fn encode(&mut self) -> io::Result<Vec<u8>> {
        let mut a = io::Cursor::new(vec![]);

        let (buf, n) = self.attrs.encode()?;

        a.write_u8(OPCODE)?;
        a.write_u8(self.depth)?;
        a.write_u16::<NativeEndian>(8 + n)?;
        a.write_u32::<NativeEndian>(self.wid)?;
        a.write_u32::<NativeEndian>(self.parent)?;
        a.write_u16::<NativeEndian>(self.x)?;
        a.write_u16::<NativeEndian>(self.y)?;
        a.write_u16::<NativeEndian>(self.width)?;
        a.write_u16::<NativeEndian>(self.height)?;
        a.write_u16::<NativeEndian>(self.border_width)?;
        a.write_u16::<NativeEndian>(self.class)?;
        a.write_u32::<NativeEndian>(self.visual)?;
        a.write(buf.as_slice())?;

        Ok(a.into_inner())
    }

    fn decode(client: Client) -> Box<Future<Item = (Client, Self::Reply), Error = io::Error>> {
        Box::new(futures::finished((client, ())))
    }
}
