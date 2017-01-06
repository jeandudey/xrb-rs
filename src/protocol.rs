//! Here basic protocol communication is described like requests and replies.

use ::std::io;

use ::futures::Future;

use ::Client;

/// An X11 Protocol request.
pub trait Request {
    type Reply: 'static;

    fn encode(&mut self) -> io::Result<Vec<u8>>;
    fn decode(client: Client) -> Box<Future<Item = (Client, Self::Reply), Error = io::Error>>;
}

/// This is used for requests that don't return a reply.
pub type VoidReply = ();

pub type ExtensionInfo = ::xproto::QueryExtensionReply;

/// An X11 Protocol extension request.
pub trait ExtensionRequest {
    type Reply: 'static;

    fn extension_name() -> &'static [u8];
    fn encode(&mut self, info: &ExtensionInfo) -> io::Result<Vec<u8>>;
    fn decode(client: Client) -> Box<Future<Item = (Client, Self::Reply), Error = io::Error>>;
}
