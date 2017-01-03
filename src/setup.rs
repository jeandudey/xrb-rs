use std::io::{self, Read, Write};
use futures::{self, Future};
use byteorder::{WriteBytesExt, ReadBytesExt, NativeEndian};
use utils::pad;
use xauth::Xauth;
use ::tokio_core;

/// Setups the connection with X11 server.
pub fn setup<A: Read + Write + 'static + Send>
    (a: A,
     auth_info: &Xauth)
     -> Box<Future<Item = (A, Setup), Error = io::Error>> {
    let req_data = try_future!(encode_setup_request(&auth_info.name, &auth_info.data));

    tokio_core::io::write_all(a, req_data)
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
        .and_then(|(a, setup_generic, buf)| {
            let mut reader = io::Cursor::new(buf);

            match setup_generic.status {
                STATUS_SUCCESS => {
                    let server_info = try!(ServerInfo::read(&mut reader, setup_generic));
                    Ok((a, Setup::Success(server_info)))
                }
                STATUS_FAILED |
                STATUS_AUTHENTICATE => {
                    let mut reason = String::new();
                    let mut take = reader.take(setup_generic.reason_len as u64);
                    try!(take.read_to_string(&mut reason));

                    if setup_generic.status == STATUS_FAILED {
                        Ok((a,
                            Setup::Failed {
                            generic: setup_generic,
                            reason: reason,
                        }))
                    } else {
                        Ok((a,
                            Setup::Authenticate {
                            generic: setup_generic,
                            reason: reason,
                        }))
                    }
                }
                _ => unreachable!(),
            }
        })
        .boxed()
}

/// This enum represents all the setup response posibilities.
pub enum Setup {
    /// This indicates that the connection was accepted.
    Success(ServerInfo),

    /// This indicates that the connection was refused.
    Failed {
        generic: SetupGeneric,
        reason: String,
    },

    /// This inidicates that further authentication is need.
    Authenticate {
        generic: SetupGeneric,
        reason: String,
    },
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
