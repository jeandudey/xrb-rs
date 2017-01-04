use ::std::io::{self, Write};
use ::byteorder::{NativeEndian, WriteBytesExt, ReadBytesExt};
use ::pad;
use ::tokio_core;
use ::futures::{self, Future};
use ::Client;

const OPCODE: u8 = 98;

/// Response of `QueryExtension` request, this is returned by the
/// `query_extension` function.
pub struct QueryExtensionResponse {
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

pub fn query_extension
    (a: Client,
     name: &[u8])
     -> Box<Future<Item = (Client, QueryExtensionResponse), Error = io::Error>> {
    let req_data = try_future!(encode_request(name));

    tokio_core::io::write_all(a, req_data)
        .and_then(|(a, _)| {
            let buf: [u8; 32] = [0u8; 32];
            tokio_core::io::read_exact(a, buf).and_then(|(a, buf)| {
                let response = try!(decode_response(buf));
                Ok((a, response))
            })
        })
        .boxed()
}

fn encode_request(name: &[u8]) -> io::Result<Vec<u8>> {
    let mut a = io::Cursor::new(vec![]);

    try!(a.write_u8(OPCODE));
    try!(a.write_u8(0));

    let n = name.len();
    let p = pad(n);
    let len = (2 + (n + p)) / 4;

    try!(a.write_u16::<NativeEndian>(len as u16));
    try!(a.write_u16::<NativeEndian>(n as u16));
    try!(a.write_u16::<NativeEndian>(0));

    try!(a.write(name));

    for _ in 0..p {
        try!(a.write_u8(0));
    }

    Ok(a.into_inner())
}

fn decode_response<B: AsRef<[u8]>>(buf: B) -> io::Result<QueryExtensionResponse> {
    let mut a = io::Cursor::new(buf);

    try!(a.read_u8());
    try!(a.read_u8());
    try!(a.read_u16::<NativeEndian>());
    try!(a.read_u32::<NativeEndian>());
    let present = try!(a.read_u8()) == 1;
    let major_opcode = try!(a.read_u8());
    let first_event = try!(a.read_u8());
    let first_error = try!(a.read_u8());

    Ok(QueryExtensionResponse {
        present: present,
        major_opcode: major_opcode,
        first_event: first_event,
        first_error: first_error,
    })
}
