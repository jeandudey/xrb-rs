use ::futures::{self, Future};
use ::tokio_core;
use ::byteorder::{WriteBytesExt, NativeEndian};
use ::std::io;
use ::Client;

const OPCODE: u8 = 1;

pub struct CreateWindow {
    wid: u32,
    parent: u32,
    class: u16,
    depth: u8,
    visual: u32,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    border_width: u16, // TODO: Add value-list
}

impl CreateWindow {
    pub fn new(wid: u32,
               parent: u32,
               class: u16,
               depth: u8,
               visual: u32,
               x: u16,
               y: u16,
               width: u16,
               height: u16,
               border_width: u16)
               -> CreateWindow {
        CreateWindow {
            wid: wid,
            parent: parent,
            class: class,
            depth: depth,
            visual: visual,
            x: x,
            y: y,
            width: width,
            height: height,
            border_width: border_width,
        }
    }

    /// Creates the request
    pub fn perform(self,
                   a: Client)
                   -> Box<Future<Item = (Client, CreateWindow), Error = io::Error>> {
        let req_data = try_future!(self.encode_request());

        tokio_core::io::write_all(a, req_data)
            .map(move |(a, _)| (a, self))
            .boxed()
    }

    fn encode_request(&self) -> io::Result<Vec<u8>> {
        let mut a = io::Cursor::new(vec![]);

        try!(a.write_u8(OPCODE));
        try!(a.write_u8(self.depth));
        try!(a.write_u16::<NativeEndian>(10)); // TODO: length
        try!(a.write_u32::<NativeEndian>(self.wid));
        try!(a.write_u32::<NativeEndian>(self.parent));
        try!(a.write_u16::<NativeEndian>(self.x));
        try!(a.write_u16::<NativeEndian>(self.y));
        try!(a.write_u16::<NativeEndian>(self.width));
        try!(a.write_u16::<NativeEndian>(self.height));
        try!(a.write_u16::<NativeEndian>(self.border_width));
        try!(a.write_u16::<NativeEndian>(self.class));
        try!(a.write_u32::<NativeEndian>(self.visual));
        // TODO: actually create value-mask and value-list
        try!(a.write_u32::<NativeEndian>(0x2 /* background-pixel */ | 0x800 /* event-mask */));
        try!(a.write_u32::<NativeEndian>(0xccffcc));
        try!(a.write_u32::<NativeEndian>(0x1 /* KeyPress */ | 0x8000 /* Exposure */));

        Ok(a.into_inner())
    }
}
