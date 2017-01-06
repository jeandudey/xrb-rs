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

const OPCODE: u8 = 99;

#[derive(Debug)]
pub struct ListExtensions;

impl Request for ListExtensions {
    type Reply = Vec<String>;

    fn encode(&self) -> io::Result<Vec<u8>> {
        let mut a = io::Cursor::new(vec![]);

        a.write_u8(OPCODE)?;
        a.write_u8(0)?;
        a.write_u16::<NativeEndian>(1)?;

        Ok(a.into_inner())
    }

    fn decode(client: Client) -> Box<Future<Item = (Client, Self::Reply), Error = io::Error>> {
        let buf: [u8; 32] = [0u8; 32];
        Box::new(tokio_core::io::read_exact(client, buf)
            .and_then(|(client, buf)| {
                let mut a = io::Cursor::new(buf);

                a.read_u8()?;
                let str_count = a.read_u8()?;
                a.read_u16::<NativeEndian>()?;
                let reply_length = a.read_u32::<NativeEndian>()?;

                Ok((client, str_count, reply_length))
            })
            .and_then(|(client, str_count, reply_length)| {
                let buf: Vec<u8> = vec![0u8; reply_length as usize * 4];
                tokio_core::io::read_exact(client, buf)
                    .map(move |(client, buf)| (client, str_count, buf))
            })
            .map(|(client, str_count, buf)| {
                let mut list = Vec::new();

                let mut iter = buf.into_iter();

                for _ in 0..str_count {
                    let mut s = String::new();
                    while let Some(c) = iter.next() {
                        match c {
                            0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 |
                            0x09 | 0x0A | 0x0B | 0x0C | 0x0D | 0x0E | 0x0F | 0x10 | 0x17 |
                            0x18 | 0x19 | 0x7F => break,
                            _ => s.push(char::from(c)),
                        }
                    }

                    list.push(s);
                }

                (client, list)
            }))
    }
}
