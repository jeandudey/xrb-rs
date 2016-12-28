use ::std::io::{self, Read, Write};
use ::futures::{self, Future};
use ::byteorder::{WriteBytesExt, ReadBytesExt, NativeEndian};
use ::utils::pad;
use ::tokio_core;
use ::xauth::Xauth;

// this code looks like shit to me, it's an monster. Drastic problem needs drastic
// solutions.

/*
/// Setups the connection with X11 server.
pub fn setup<A: Read + Write + 'static + Send>(a: A, auth_info: &Xauth) -> Box<Future<Item=(A, Setup), Error=io::Error>> {
    let req_data = try_future!(encode_setup_request(&auth_info.name, &auth_info.data));

    tokio_core::io::write_all(a, req_data).and_then(|(a, _)| {
        tokio_core::io::flush(a)
    }).and_then(|a| {
        let status_buf: [u8; 2] = [0; 2];
        tokio_core::io::read(a, status_buf).map((|(a, status_buf, bytes_read)| {
            assert_eq!(bytes_read, 2);
            (a, status_buf[0])
        }))
    }).and_then(|(a, status)| {
        let size: u64 = match status {
            STATUS_FAILED | STATUS_AUTHENTICATE => 6,
            STATUS_SUCCESS => 38,
            _ => return Err(invalid_status()),
        };

        Ok((a, status, size))
    }).and_then(|(a, status, size)| {
        let take = a.take(size);
        let buf: Vec<u8> = Vec::new();
        tokio_core::io::read_to_end(take, buf).map(move |(take, buf)| {
            (take.into_inner(), buf, status)
        })
    }).and_then(|(a, buf, status)| {
        // Read byte-size constant data

        let mut constant_size_reader = io::Cursor::new(buf);

        let header = try!(SetupHeader::read(&mut constant_size_reader));

        if status == STATUS_SUCCESS {
            Ok((a, Some(constant_size_reader), status, header))
        } else {
            Ok((a, None, status, header))
        }
    }).and_then(|(a, constant_size_reader, status, header)| {
        let additional_data_buf: Vec<u8> = Vec::new();
        let take = a.take(header.additional_data_len as u64 * 4);
        panic!("additional_data: {}", header.additional_data_len);
        tokio_core::io::read_to_end(take, additional_data_buf).map(move|(take, additional_data_buf)| {
            let a = take.into_inner();

            if status == STATUS_SUCCESS {
                (a, additional_data_buf, constant_size_reader, status, header)
            } else {
                (a, additional_data_buf, constant_size_reader, status, header)
            }
        })
    }).and_then(|(a, additional_data_buf, constant_size_reader, status, header)| {
        // Read additional data

        match status {
            STATUS_FAILED | STATUS_AUTHENTICATE => {
                let reader = io::Cursor::new(additional_data_buf);

                let mut reason = String::new();
                try!(reader.take(header.reason_len as u64).read_to_string(&mut reason));

                if status == STATUS_FAILED {
                    Ok((a, Setup::Failed { header: header, reason: reason }))
                } else {
                    Ok((a, Setup::Authenticate { header: header, reason: reason }))
                }
            },
            STATUS_SUCCESS => {
                //let mut reader = constant_size_reader;

                Ok((a, Setup::Success {
                    header: header,
                }))
            },
            _ => Err(invalid_status()),
        }
    }).boxed()
}*/


/// Setups the connection with X11 server.
pub fn setup<A: Read + Write + 'static + Send>(a: A, auth_info: &Xauth) -> Box<Future<Item=(A, Setup), Error=io::Error>> {
    let req_data = try_future!(encode_setup_request(&auth_info.name, &auth_info.data));

    tokio_core::io::write_all(a, req_data).and_then(|(a, _)| {
        let buf = Vec::new();
        tokio_core::io::read_to_end(a, buf)
    }).map(|(a, buf)| {
        panic!("len: {}", buf.len());
        (a, Setup::Success {
            header: SetupHeader {
                reason_len: 0,
                protocol_major_version: 0,
                protocol_minor_version: 0,
                additional_data_len: 0,
            }
        })
    }).boxed()
}

fn invalid_status() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, "Not valid status")
}

/// This enum represents all the setup response posibilities.
pub enum Setup {
    /// This indicates that the connection was accepted.
    Success {
        header: SetupHeader,
    },

    /// This indicates that the connection was refused.
    Failed {
        header: SetupHeader,
        reason: String,
    },

    /// This inidicates that further authentication is need.
    Authenticate {
        header: SetupHeader,
        reason: String,
    },
}

/// The setup header
#[derive(Debug)]
pub struct SetupHeader {
    /// If any error ocurred this indicates the reason string length.
    pub reason_len: u8,

    /// X protocol major version.
    pub protocol_major_version: u16,

    /// X protocol minor version.
    pub protocol_minor_version: u16,

    /// Additional data length after this header.
    pub additional_data_len: u16,
}

impl SetupHeader {
    fn read<A: Read>(reader: &mut A) -> io::Result<SetupHeader> {
        let reason_len = try!(reader.read_u8());
        let major_version = try!(reader.read_u16::<NativeEndian>());
        let minor_version = try!(reader.read_u16::<NativeEndian>());
        let additional_data_len = try!(reader.read_u16::<NativeEndian>());

        Ok(SetupHeader {
            reason_len: reason_len,
            protocol_major_version: major_version,
            protocol_minor_version: minor_version,
            additional_data_len: additional_data_len,
        })
    }
}

// The encoding of the request can be found at:
// https://www.x.org/releases/X11R7.7/doc/xproto/x11protocol.html#Encoding::Connection_Setup
fn encode_setup_request(auth_name: &Vec<u8>, auth_data: &Vec<u8>) -> io::Result<Vec<u8>> {
    let mut writer = io::Cursor::new(vec![]);

    try!(writer.write_u8(BYTE_ORDER));
    try!(writer.write_u8(0)); // pad
    try!(writer.write_u16::<NativeEndian>(11)); // protocol-major-version
    try!(writer.write_u16::<NativeEndian>(0)); // protocol-minor-version
    try!(writer.write_u16::<NativeEndian>(auth_name.len() as u16));
    try!(writer.write_u16::<NativeEndian>(auth_data.len() as u16));
    try!(writer.write_u16::<NativeEndian>(0)); // pad

    try!(writer.write(auth_name.as_slice()));
    for _ in 0..pad(auth_name.len()) {
        try!(writer.write_u8(0));
    }


    try!(writer.write(auth_data.as_slice()));
    for _ in 0..pad(auth_data.len()) {
        try!(writer.write_u8(0));
    }

    Ok(writer.into_inner())
}

#[cfg(target_endian = "big")]
const BYTE_ORDER: u8 = 0x42; 

#[cfg(target_endian = "little")]
const BYTE_ORDER: u8 = 0x6C; 

pub const STATUS_FAILED: u8 = 0;
pub const STATUS_SUCCESS: u8 = 1;
pub const STATUS_AUTHENTICATE: u8 = 2;
