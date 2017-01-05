#![deny(trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

//! xrb-rs - X Rust Bindings
//!
//! This crate is a proof of concept, it's not ready for production yet, but
//! in the future this crate aims to be a good alternative to Xlib and XCB.
//!
//! # Usage
//! As this crate isn't published on crates.io for obvious reasons you need
//! to add it to cargo as a git repository.
//!
//! ```toml
//! [dependencies]
//! xrb = { git = "https://www.github.com/jeandudey/xrb-rs/" }
//! ```
//!
//! And put this obvious code to your crate root.
//!
//! ```rust
//! extern crate xrb;
//! ```
//!
//! Now you can start using it.
//!
//! # Connecting
//! The only supported method of connection is trough Unix Domain Sockets,
//! because *TCP* is *mainstream* and a bunch of **geeks** (nerds) don't
//! want to use it because they say it's unsafe.

extern crate tokio_core;
extern crate tokio_uds;
extern crate futures;
extern crate byteorder;
extern crate xauth;

use tokio_core::reactor::Handle;
use tokio_uds::UnixStream;
use std::io::{self, Read, Write};
use std::collections::HashMap;
use byteorder::{ReadBytesExt, WriteBytesExt, NativeEndian};
use futures::Future;

/// `Xauth` is used to get authentication information.
pub use xauth::Xauth;

#[macro_use]
mod macros;
mod utils;

mod setup_error;
pub use setup_error::*;

pub mod protocol;

/// Function used to calculate pad for unused bytes.
pub fn pad(e: usize) -> usize {
    ((4 - (e % 4)) % 4)
}

/// An X11 Protocol client.
pub struct Client {
    socket: UnixStream,
    server_info: ServerInfo,
    extensions: HashMap<&'static [u8], protocol::ExtensionInfo>,
}

impl Client {
    /// This function creates a future that when it's done it will resolve
    /// to a connected X11 client.
    ///
    /// # Panics
    /// A panic will be thrown if you pass `None` as display and the `DISPLAY`
    /// environment variable is not found.
    pub fn connect<D>(display: D,
                      auth_info: &Xauth,
                      handle: Handle)
                      -> Box<Future<Item = Self, Error = SetupError>>
        where D: Into<Option<u16>>
    {
        let disp = if let Some(d) = display.into() {
            d
        } else {
            utils::get_default_display_number().expect("Failed to get DISPLAY environment variable")
        };

        let path = format!("/tmp/.X11-unix/X{}", disp);
        let socket = UnixStream::connect(&path, &handle).unwrap();

        let req_data = encode_setup_request(&auth_info.name, &auth_info.data).unwrap();

        tokio_core::io::write_all(socket, req_data)
            .and_then(|(a, _)| {
                let buf: [u8; 8] = [0u8; 8];
                tokio_core::io::read_exact(a, buf).and_then(|(a, buf)| {
                    let mut reader = io::Cursor::new(buf);
                    let setup_generic = try!(SetupGeneric::read(&mut reader));
                    Ok((a, setup_generic))
                })
            })
            .and_then(|(a, setup_generic)| {
                let buf: Vec<u8> = vec![0u8; setup_generic.additional_data_len as usize * 4];
                tokio_core::io::read_exact(a, buf).map(move |(a, buf)| (a, setup_generic, buf))
            })
            .map_err(SetupError::from)
            .and_then(|(a, setup_generic, buf)| {
                let mut reader = io::Cursor::new(buf);

                match setup_generic.status {
                    STATUS_SUCCESS => {
                        let server_info = ServerInfo::read(&mut reader, setup_generic)?;
                        Ok((a, server_info))
                    }
                    STATUS_FAILED |
                    STATUS_AUTHENTICATE => {
                        let mut reason = String::new();
                        let mut take = reader.take(setup_generic.reason_len as u64);
                        take.read_to_string(&mut reason)?;

                        if setup_generic.status == STATUS_FAILED {
                            Err(SetupError::Failed(reason))
                        } else {
                            Err(SetupError::Authenticate(reason))
                        }
                    }
                    _ => unreachable!(),
                }
            })
            .map(|(socket, server_info)| {
                Client {
                    socket: socket,
                    server_info: server_info,
                    extensions: HashMap::new(),
                }
            })
            .boxed()
    }

    pub fn perform<Req: protocol::Request>
        (self,
         request: Req)
         -> Box<Future<Item = (Self, <Req as protocol::Request>::Reply), Error = io::Error>> {
        let req_data = request.encode().unwrap();
        Box::new(tokio_core::io::write_all(self, req_data)
            .and_then(|(client, _)| Req::decode(client)))
    }

    pub fn perform_ex<Req: protocol::ExtensionRequest + 'static>
        (self,
         request: Req)
         -> Box<Future<Item = (Self, <Req as protocol::ExtensionRequest>::Reply), Error = io::Error>> {
        let extension_name = Req::extension_name();
        let maybe_extension = {
            self.extensions.get(extension_name).cloned()
        };

        if let Some(info) = maybe_extension {
            let req_data = request.encode(&info).unwrap();
            Box::new(tokio_core::io::write_all(self, req_data)
                .and_then(|(client, _)| Req::decode(client)))
        } else {
            Box::new(self.perform(xproto::QueryExtension { name: extension_name.to_owned() })
                .and_then(move |(mut client, info)| {
                    client.extensions.insert(extension_name, info.clone());

                    let req_data = request.encode(&info).unwrap();
                    tokio_core::io::write_all(client, req_data)
                        .and_then(|(client, _)| Req::decode(client))
                }))
        }
    }

    /// Returns the server information structure.
    pub fn get_server_info(&self) -> &ServerInfo {
        &self.server_info
    }
}

impl io::Read for Client {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.socket.read(buf)
    }
}

impl io::Write for Client {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.socket.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.socket.flush()
    }
}

impl tokio_core::io::Io for Client {
    fn poll_read(&mut self) -> futures::Async<()> {
        self.socket.poll_read()
    }

    fn poll_write(&mut self) -> futures::Async<()> {
        self.socket.poll_write()
    }
}

impl<'a> io::Read for &'a Client {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (&self.socket).read(buf)
    }
}

impl<'a> io::Write for &'a Client {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (&self.socket).write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        (&self.socket).flush()
    }
}

impl<'a> tokio_core::io::Io for &'a Client {
    fn poll_read(&mut self) -> futures::Async<()> {
        self.socket.poll_read()
    }

    fn poll_write(&mut self) -> futures::Async<()> {
        self.socket.poll_write()
    }
}

/// Information received if the connection is accepted.
#[derive(Debug)]
pub struct ServerInfo {
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub release_number: u32,
    pub resource_id_base: u32,
    pub resource_id_mask: u32,
    pub motion_buffer_size: u32,
    pub maximum_request_length: u16,
    pub image_byte_order: u8,
    pub bitmap_format_bit_order: u8,
    pub bitmap_format_scanline_unit: u8,
    pub bitmap_format_scanline_pad: u8,
    pub min_keycode: u8,
    pub max_keycode: u8,

    /// Vendor string of server.
    pub vendor: String,

    /// List of Pixmap formats.
    pub pixmap_formats: Vec<Format>,

    /// List of screens.
    pub roots: Vec<Screen>,
}

impl ServerInfo {
    fn read<A: Read>(a: &mut A, setup_generic: SetupGeneric) -> io::Result<ServerInfo> {
        let release_number = try!(a.read_u32::<NativeEndian>());
        let resource_id_base = try!(a.read_u32::<NativeEndian>());
        let resource_id_mask = try!(a.read_u32::<NativeEndian>());
        let motion_buffer_size = try!(a.read_u32::<NativeEndian>());
        let vendor_len = try!(a.read_u16::<NativeEndian>());
        let maximum_request_length = try!(a.read_u16::<NativeEndian>());
        let roots_len = try!(a.read_u8());
        let pixmap_formats_len = try!(a.read_u8());
        let image_byte_order = try!(a.read_u8());
        let bitmap_format_bit_order = try!(a.read_u8());
        let bitmap_format_scanline_unit = try!(a.read_u8());
        let bitmap_format_scanline_pad = try!(a.read_u8());
        let min_keycode = try!(a.read_u8());
        let max_keycode = try!(a.read_u8());
        try!(a.read_u32::<NativeEndian>());

        let vendor = {
            let mut v_str = String::new();
            let mut take = a.by_ref().take(vendor_len as u64);
            try!(take.read_to_string(&mut v_str));
            v_str
        };

        for _ in 0..pad(vendor_len as usize) {
            try!(a.read_u8());
        }

        let mut pixmap_formats = Vec::new();
        for _ in 0..pixmap_formats_len {
            pixmap_formats.push(try!(Format::read(a)));
        }

        let mut roots = Vec::new();
        for _ in 0..roots_len {
            roots.push(try!(Screen::read(a)));
        }


        Ok(ServerInfo {
            protocol_major_version: setup_generic.protocol_major_version,
            protocol_minor_version: setup_generic.protocol_minor_version,
            release_number: release_number,
            resource_id_base: resource_id_base,
            resource_id_mask: resource_id_mask,
            motion_buffer_size: motion_buffer_size,
            maximum_request_length: maximum_request_length,
            image_byte_order: image_byte_order,
            bitmap_format_bit_order: bitmap_format_bit_order,
            bitmap_format_scanline_unit: bitmap_format_scanline_unit,
            bitmap_format_scanline_pad: bitmap_format_scanline_pad,
            min_keycode: min_keycode,
            max_keycode: max_keycode,
            vendor: vendor,
            pixmap_formats: pixmap_formats,
            roots: roots,
        })
    }
}

/// The setup header
#[derive(Debug)]
pub struct SetupGeneric {
    /// Success status.
    pub status: u8,

    /// If any error ocurred this indicates the reason string length.
    pub reason_len: u8,

    /// X protocol major version.
    pub protocol_major_version: u16,

    /// X protocol minor version.
    pub protocol_minor_version: u16,

    /// Additional data length after this header.
    pub additional_data_len: u16,
}

impl SetupGeneric {
    fn read<A: Read>(reader: &mut A) -> io::Result<SetupGeneric> {
        let status = try!(reader.read_u8());
        let reason_len = try!(reader.read_u8());
        let major_version = try!(reader.read_u16::<NativeEndian>());
        let minor_version = try!(reader.read_u16::<NativeEndian>());
        let additional_data_len = try!(reader.read_u16::<NativeEndian>());

        Ok(SetupGeneric {
            status: status,
            reason_len: reason_len,
            protocol_major_version: major_version,
            protocol_minor_version: minor_version,
            additional_data_len: additional_data_len,
        })
    }
}

fn encode_setup_request<D: AsRef<[u8]>>(auth_name: D, auth_data: D) -> io::Result<Vec<u8>> {
    let name = auth_name.as_ref();
    let data = auth_data.as_ref();

    let mut writer = io::Cursor::new(vec![]);

    try!(writer.write_u8(BYTE_ORDER));
    try!(writer.write_u8(0)); // pad
    try!(writer.write_u16::<NativeEndian>(11)); // protocol-major-version
    try!(writer.write_u16::<NativeEndian>(0)); // protocol-minor-version
    try!(writer.write_u16::<NativeEndian>(name.len() as u16));
    try!(writer.write_u16::<NativeEndian>(data.len() as u16));
    try!(writer.write_u16::<NativeEndian>(0)); // pad

    try!(writer.write(name));
    for _ in 0..pad(name.len()) {
        try!(writer.write_u8(0));
    }


    try!(writer.write(data));
    for _ in 0..pad(data.len()) {
        try!(writer.write_u8(0));
    }

    Ok(writer.into_inner())
}

#[cfg(target_endian = "big")]
const BYTE_ORDER: u8 = b'B';

#[cfg(target_endian = "little")]
const BYTE_ORDER: u8 = b'l';

pub const STATUS_FAILED: u8 = 0;
pub const STATUS_SUCCESS: u8 = 1;
pub const STATUS_AUTHENTICATE: u8 = 2;

#[derive(Debug)]
pub struct Format {
    pub depth: u8,
    pub bits_per_pixel: u8,
    pub scanline_pad: u8,
}

impl Format {
    fn read<A: Read>(a: &mut A) -> io::Result<Format> {
        let depth = try!(a.read_u8());
        let bits_per_pixel = try!(a.read_u8());
        let scanline_pad = try!(a.read_u8());
        try!(a.read_exact(&mut [0; 5]));

        Ok(Format {
            depth: depth,
            bits_per_pixel: bits_per_pixel,
            scanline_pad: scanline_pad,
        })
    }
}

#[derive(Debug)]
pub enum BackingStores {
    Never,
    WhenMapped,
    Always,
}

impl From<u8> for BackingStores {
    fn from(backing_stores: u8) -> BackingStores {
        match backing_stores {
            0 => BackingStores::Never,
            1 => BackingStores::WhenMapped,
            2 => BackingStores::Always,
            _ => panic!("invalid BackingStores"),
        }
    }
}

#[derive(Debug)]
pub struct Screen {
    pub root: u32,
    pub default_colormap: u32,
    pub white_pixel: u32,
    pub black_pixel: u32,
    pub current_input_masks: u32,
    pub width_pixels: u16,
    pub height_pixels: u16,
    pub width_millimeters: u16,
    pub height_millimeters: u16,
    pub min_installed_maps: u16,
    pub max_installed_maps: u16,
    pub root_visual: u32,
    pub backing_stores: BackingStores,
    pub save_unders: bool,
    pub root_depth: u8,
    pub allowed_depths: Vec<Depth>,
}

impl Screen {
    fn read<A: Read>(a: &mut A) -> io::Result<Screen> {
        let root = try!(a.read_u32::<NativeEndian>());
        let default_colormap = try!(a.read_u32::<NativeEndian>());
        let white_pixel = try!(a.read_u32::<NativeEndian>());
        let black_pixel = try!(a.read_u32::<NativeEndian>());
        let current_input_masks = try!(a.read_u32::<NativeEndian>());
        let width_pixels = try!(a.read_u16::<NativeEndian>());
        let height_pixels = try!(a.read_u16::<NativeEndian>());
        let width_millimeters = try!(a.read_u16::<NativeEndian>());
        let height_millimeters = try!(a.read_u16::<NativeEndian>());
        let min_installed_maps = try!(a.read_u16::<NativeEndian>());
        let max_installed_maps = try!(a.read_u16::<NativeEndian>());
        let root_visual = try!(a.read_u32::<NativeEndian>());
        let backing_stores = try!(a.read_u8());
        let save_unders = try!(a.read_u8()) != 0;
        let root_depth = try!(a.read_u8());
        let depth_count = try!(a.read_u8());

        let mut allowed_depths = Vec::new();
        for _ in 0..depth_count {
            allowed_depths.push(try!(Depth::read(a)));
        }

        Ok(Screen {
            root: root,
            default_colormap: default_colormap,
            white_pixel: white_pixel,
            black_pixel: black_pixel,
            current_input_masks: current_input_masks,
            width_pixels: width_pixels,
            height_pixels: height_pixels,
            width_millimeters: width_millimeters,
            height_millimeters: height_millimeters,
            min_installed_maps: min_installed_maps,
            max_installed_maps: max_installed_maps,
            root_visual: root_visual,
            backing_stores: BackingStores::from(backing_stores),
            save_unders: save_unders,
            root_depth: root_depth,
            allowed_depths: allowed_depths,
        })
    }
}

#[derive(Debug)]
pub struct Depth {
    pub depth: u8,
    pub visuals: Vec<Visual>,
}

impl Depth {
    fn read<A: Read>(a: &mut A) -> io::Result<Depth> {
        let depth = try!(a.read_u8());
        try!(a.read_u8());
        let visual_count = try!(a.read_u16::<NativeEndian>());
        try!(a.read_u32::<NativeEndian>());

        let mut visuals = Vec::new();
        for _ in 0..visual_count {
            visuals.push(try!(Visual::read(a)));
        }

        Ok(Depth {
            depth: depth,
            visuals: visuals,
        })
    }
}

#[derive(Debug)]
pub enum VisualClass {
    StaticGray,
    GrayScale,
    StaticColor,
    PseudoColor,
    TrueColor,
    DirectColor,
}

impl From<u8> for VisualClass {
    fn from(class: u8) -> VisualClass {
        match class {
            0 => VisualClass::StaticGray,
            1 => VisualClass::GrayScale,
            2 => VisualClass::StaticColor,
            3 => VisualClass::PseudoColor,
            4 => VisualClass::TrueColor,
            5 => VisualClass::DirectColor,
            _ => panic!("Invalid VisualClass"),
        }
    }
}

#[derive(Debug)]
pub struct Visual {
    pub id: u32,
    pub class: VisualClass,
    pub bits_per_rgb_value: u8,
    pub colormap_entries: u16,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
}

impl Visual {
    fn read<A: Read>(a: &mut A) -> io::Result<Visual> {
        let id = try!(a.read_u32::<NativeEndian>());
        let class = try!(a.read_u8());
        let bits_per_rgb_value = try!(a.read_u8());
        let colormap_entries = try!(a.read_u16::<NativeEndian>());
        let red_mask = try!(a.read_u32::<NativeEndian>());
        let green_mask = try!(a.read_u32::<NativeEndian>());
        let blue_mask = try!(a.read_u32::<NativeEndian>());
        try!(a.read_u32::<NativeEndian>());

        Ok(Visual {
            id: id,
            class: VisualClass::from(class),
            bits_per_rgb_value: bits_per_rgb_value,
            colormap_entries: colormap_entries,
            red_mask: red_mask,
            green_mask: green_mask,
            blue_mask: blue_mask,
        })
    }
}

pub mod xproto;
pub mod xc_misc;
