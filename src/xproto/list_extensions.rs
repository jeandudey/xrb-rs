use ::std::io;
use ::std::io::Read;

use ::futures::Future;
use ::byteorder::NativeEndian;
use ::byteorder::WriteBytesExt;
use ::byteorder::ReadBytesExt;
use ::tokio_io;

use ::protocol::Request;
use ::Client;

const OPCODE: u8 = 99;

#[derive(Debug)]
pub struct ListExtensions;

impl Request for ListExtensions {
    type Reply = Vec<String>;

    fn encode(&mut self) -> io::Result<Vec<u8>> {
        let mut a = io::Cursor::new(vec![]);

        a.write_u8(OPCODE)?;
        a.write_u8(0)?;
        a.write_u16::<NativeEndian>(1)?;

        Ok(a.into_inner())
    }

    fn decode(client: Client) -> Box<Future<Item = (Client, Self::Reply), Error = io::Error>> {
        let buf: [u8; 32] = [0u8; 32];
        Box::new(tokio_io::io::read_exact(client, buf)
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
                tokio_io::io::read_exact(client, buf)
                    .map(move |(client, buf)| (client, str_count, buf))
            })
            .and_then(|(client, str_count, buf)| {
                let mut a = io::Cursor::new(buf);
                let mut list = Vec::new();

                for _ in 0..str_count {
                    let size = a.read_u8()? as u64;
                    let mut s = String::new();
                    {
                        a.by_ref().take(size).read_to_string(&mut s)?;
                    }

                    list.push(s);
                }

                Ok((client, list))
            }))
    }
}
