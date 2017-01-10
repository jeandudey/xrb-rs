//! Here basic protocol communication is described like requests and replies.

use ::std::io;
use ::std::io::Read;

use ::futures::Future;
use ::byteorder::NativeEndian;
use ::byteorder::ReadBytesExt;

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

/// A potocol error.
#[derive(Debug)]
pub enum Error {
    /// The major or minor opcode does not specify a valid request.
    Request {
        sequence_number: u16,
        minor_opcode: u16,
        major_opcode: u8,
    },

    /// Some numeric value falls outside the range of values accepted by the
    /// request. Unless a specific range is specified for an argument, the full
    /// range defined by the argument's type is accepted. Any argument defined
    /// as a set of alternatives typically can generate this error (due to the encoding).
    Value {
        sequence_number: u16,
        bad_value: u32,
        minor_opcode: u16,
        major_opcode: u8,
    },

    /// A value for a `Window` argument does not name a defined `Window`.
    Window {
        sequence_number: u16,
        bad_resource_id: u32,
        minor_opcode: u16,
        major_opcode: u8,
    },

    /// A value for a `Pixmap` argument does not name a defined `Pixmap`.
    Pixmap {
        sequence_number: u16,
        bad_resource_id: u32,
        minor_opcode: u16,
        major_opcode: u8,
    },

    /// A value for an `Atom` argument does not name a defined `Atom`.
    Atom {
        sequence_number: u16,
        bad_atom_id: u32,
        minor_opcode: u16,
        major_opcode: u8,
    },

    /// A value for a `Cursor` argument does not name a defined `Cursor`.
    Cursor {
        sequence_number: u16,
        bad_resource_id: u32,
        minor_opcode: u16,
        major_opcode: u8,
    },

    /// A value for a `Font` argument does not name a defined `Font`. A value
    /// for a `Fontable` argument does not name a defined `Font` or a defined
    /// `GContext`.
    Font {
        sequence_number: u16,
        bad_resource_id: u32,
        minor_opcode: u16,
        major_opcode: u8,
    },

    /// An `InputOnly` window is used as a `Drawable`. In a graphics request,
    /// the `GContext` argument does not have the same root and depth as the
    /// destination `Drawable` argument. Some argument (or pair of arguments)
    /// has the correct type and range, but it fails to match in some other way
    /// required by the request.
    Match {
        sequence_number: u16,
        minor_opcode: u16,
        major_opcode: u8,
    },

    /// A value for a `Drawable` argument does not name a defined `Window` or
    /// `Pixmap`.
    Drawable {
        sequence_number: u16,
        bad_resource_id: u32,
        minor_opcode: u16,
        major_opcode: u8,
    },

    /// An attempt is made to grab a key/button combination already grabbed by
    /// another client. An attempt is made to free a colormap entry not
    /// allocated by the client or to free an entry in a colormap that was
    /// created with all entries writable. An attempt is made to store into a
    /// read-only or an unallocated colormap entry. An attempt is made to
    /// modify the access control list from other than the local host (or
    /// otherwise authorized client). An attempt is made to select an event type
    /// that only one client can select at a time when another client has
    /// already selected it.
    Access {
        sequence_number: u16,
        minor_opcode: u16,
        major_opcode: u8,
    },

    /// The server failed to allocate the requested resource. Note that the
    /// explicit listing of `Alloc` errors in request only covers allocation
    /// errors at a very coarse level and is not intended to cover all cases of
    /// a server running out of allocation space in the middle of service. The
    /// semantics when a server runs out of allocation space are left
    /// unspecified, but a server may generate an Alloc error on any request
    /// for this reason, and clients should be prepared to receive such errors
    /// and handle or discard them.
    Alloc {
        sequence_number: u16,
        minor_opcode: u16,
        major_opcode: u8,
    },

    /// A value for a `Colormap` argument does not name a defined `Colormap`.
    Colormap {
        sequence_number: u16,
        bad_resource_id: u32,
        minor_opcode: u16,
        major_opcode: u8,
    },

    /// A value for a `GContext` argument does not name a defined `GContext`.
    GContext {
        sequence_number: u16,
        bad_resource_id: u32,
        minor_opcode: u16,
        major_opcode: u8,
    },

    /// The value chosen for a resource identifier either is not included in the
    /// range assigned to the client or is already in use.
    IDChoice {
        sequence_number: u16,
        bad_resource_id: u32,
        minor_opcode: u16,
        major_opcode: u8,
    },

    /// A font or color of the specified name does not exist.
    Name {
        sequence_number: u16,
        minor_opcode: u16,
        major_opcode: u8,
    },

    /// The length of a request is shorter or longer than that required to
    /// minimally contain the arguments. The length of a request exceeds the
    /// maximum length accepted by the server.
    Length {
        sequence_number: u16,
        minor_opcode: u16,
        major_opcode: u8,
    },

    /// The server does not implement some aspect of the request. A server that
    /// generates this error for a core request is deficient. As such, this
    /// error is not listed for any of the requests, but clients should be
    /// prepared to receive such errors and handle or discard them.
    Implementation {
        sequence_number: u16,
        minor_opcode: u16,
        major_opcode: u8,
    },

    /// An I/O error occurred during writing/reading.
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}
