//! Here basic protocol communication is described like requests and replies.

use ::std::io;

use ::futures::Future;

use ::Client;

/// An X11 Protocol request.
pub trait Request {
    type Reply: 'static;

    fn encode(&self) -> io::Result<Vec<u8>>;
    fn decode(client: Client) -> Box<Future<Item = (Client, Self::Reply), Error = io::Error>>;
}

/// This is used for requests that don't return a reply.
pub type VoidReply = ();
