use ::futures::{self, Future};
use ::tokio_core;
use ::byteorder::{WriteBytesExt, NativeEndian};
use ::std::io;
use ::Client;

const OPCODE: u8 = 8;

pub struct MapWindow {
    window: u32,
}

impl MapWindow {
    pub fn new(window: u32) -> Self {
        MapWindow { window: window }
    }

    /// Creates the request
    pub fn perform(self, a: Client) -> Box<Future<Item = (Client, MapWindow), Error = io::Error>> {
        let req_data = try_future!(self.encode_request());

        tokio_core::io::write_all(a, req_data)
            .map(move |(a, _)| (a, self))
            .boxed()
    }

    fn encode_request(&self) -> io::Result<Vec<u8>> {
        let mut a = Vec::new();
        let request_size: u16 = 2;

        try!(a.write_u8(OPCODE));
        try!(a.write_u8(0));
        try!(a.write_u16::<NativeEndian>(request_size));
        try!(a.write_u32::<NativeEndian>(self.window));

        Ok(a)
    }
}
