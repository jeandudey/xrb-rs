//! This module contains all X11 standard requests.

use std::io;

use byteorder::NativeEndian;
use byteorder::WriteBytesExt;

macro_rules! declare_requests {
    ($($request:ident),+) => {
        $(
            mod $request;
            pub use self::$request::*;
        )+
    }
}

declare_requests! {
    create_window,
    map_window,
    query_extension,
    list_extensions
}

/// A X11 Protocol ID

pub type Xid = u32;

/// A window identifier.
pub type Window = Xid;

/// A Pixmap identifier.
pub type Pixmap = u32;

/// A Colormap identifier.
pub type Colormap = u32;

/// A Cursor identifier.
pub type Cursor = u32;

/// Bit gravity.
#[derive(Debug, Clone, Copy)]
pub enum BitGravity {
    Forget,
    NorthWest,
    North,
    NorthEast,
    West,
    Center,
    East,
    SouthWest,
    South,
    SouthEast,
    Static,
}

impl Into<u32> for BitGravity {
    fn into(self) -> u32 {
        match self {
            BitGravity::Forget => 0,
            BitGravity::NorthWest => 1,
            BitGravity::North => 2,
            BitGravity::NorthEast => 3,
            BitGravity::West => 4,
            BitGravity::Center => 5,
            BitGravity::East => 6,
            BitGravity::SouthWest => 7,
            BitGravity::South => 8,
            BitGravity::SouthEast => 9,
            BitGravity::Static => 10,
        }
    }
}

/// Window gravity.
#[derive(Debug, Clone, Copy)]
pub enum WinGravity {
    Unmap,
    NorthWest,
    North,
    NorthEast,
    West,
    Center,
    East,
    SouthWest,
    South,
    SouthEast,
    Static,
}

impl Into<u32> for WinGravity {
    fn into(self) -> u32 {
        match self {
            WinGravity::Unmap => 0,
            WinGravity::NorthWest => 1,
            WinGravity::North => 2,
            WinGravity::NorthEast => 3,
            WinGravity::West => 4,
            WinGravity::Center => 5,
            WinGravity::East => 6,
            WinGravity::SouthWest => 7,
            WinGravity::South => 8,
            WinGravity::SouthEast => 9,
            WinGravity::Static => 10,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BackingStore {
    NotUseful,
    WhenMapped,
    Always,
}

impl Into<u32> for BackingStore {
    fn into(self) -> u32 {
        match self {
            BackingStore::NotUseful => 0,
            BackingStore::WhenMapped => 1,
            BackingStore::Always => 2,
        }
    }
}

pub type Event = u32;
pub const EVENT_KEY_PRESS: Event = 0x00000001;
pub const EVENT_KEY_RELEASE: Event = 0x00000002;
pub const EVENT_BUTTON_PRESS: Event = 0x00000004;
pub const EVENT_BUTTON_RELEASE: Event = 0x00000008;
pub const EVENT_ENTER_WINDOW: Event = 0x00000010;
pub const EVENT_LEAVE_WINDOW: Event = 0x00000020;
pub const EVENT_POINTER_MOTION: Event = 0x00000040;
pub const EVENT_POINTER_MOTION_HINT: Event = 0x00000080;
pub const EVENT_BUTTON1_MOTION: Event = 0x00000100;
pub const EVENT_BUTTON2_MOTION: Event = 0x00000200;
pub const EVENT_BUTTON3_MOTION: Event = 0x00000400;
pub const EVENT_BUTTON4_MOTION: Event = 0x00000800;
pub const EVENT_BUTTON5_MOTION: Event = 0x00001000;
pub const EVENT_BUTTON_MOTION: Event = 0x00002000;
pub const EVENT_KEYMAP_STATE: Event = 0x00004000;
pub const EVENT_EXPOSURE: Event = 0x00008000;
pub const EVENT_VISIBILITY_CHANGE: Event = 0x00010000;
pub const EVENT_STRUCTURE_NOTIFY: Event = 0x00020000;
pub const EVENT_RESIZE_REDIRECT: Event = 0x00040000;
pub const EVENT_SUBSTRUCTURE_NOTIFY: Event = 0x00080000;
pub const EVENT_SUBSTRUCTURE_REDIRECT: Event = 0x00100000;
pub const EVENT_FOCUS_CHANGE: Event = 0x00200000;
pub const EVENT_PROPERTY_CHANGE: Event = 0x00400000;
pub const EVENT_COLORMAP_CHANGE: Event = 0x00800000;
pub const EVENT_OWNERGRAB_BUTTON: Event = 0x01000000;
pub const EVENT_UNUSED: Event = 0xFE000000;

pub type DeviceEvent = u32;
pub const DEVICE_EVENT_BUTTON_PRESS: DeviceEvent = 0x00000004;
pub const DEVICE_EVENT_BUTTON_RELEASE: DeviceEvent = 0x00000008;
pub const DEVICE_EVENT_ENTER_WINDOW: DeviceEvent = 0x00000010;
pub const DEVICE_EVENT_LEAVE_WINDOW: DeviceEvent = 0x00000020;
pub const DEVICE_EVENT_POINTER_MOTION: DeviceEvent = 0x00000040;
pub const DEVICE_EVENT_POINTER_MOTION_HINT: DeviceEvent = 0x00000080;
pub const DEVICE_EVENT_BUTTON1_MOTION: DeviceEvent = 0x00000100;
pub const DEVICE_EVENT_BUTTON2_MOTION: DeviceEvent = 0x00000200;
pub const DEVICE_EVENT_BUTTON3_MOTION: DeviceEvent = 0x00000400;
pub const DEVICE_EVENT_BUTTON4_MOTION: DeviceEvent = 0x00000800;
pub const DEVICE_EVENT_BUTTON5_MOTION: DeviceEvent = 0x00001000;
pub const DEVICE_EVENT_BUTTON_MOTION: DeviceEvent = 0x00002000;
pub const DEVICE_EVENT_KEYMAP_STATE: DeviceEvent = 0x00004000;
pub const DEVICE_EVENT_UNUSED: DeviceEvent = 0xFFFF8003;

#[derive(Debug, Clone, Copy)]
pub struct WindowAttributes {
    background_pixmap: Pixmap,
    background_pixel: u32,
    border_pixmap: Pixmap,
    border_pixel: u32,
    bit_gravity: BitGravity,
    win_gravity: WinGravity,
    backing_store: BackingStore,
    backing_planes: u32,
    backing_pixel: u32,
    override_redirect: bool,
    save_under: bool,
    event_mask: Event,
    do_not_propagate_mask: DeviceEvent,
    colormap: Colormap,
    cursor: Cursor,

    value_mask: u32,
}

impl WindowAttributes {
    pub fn new() -> WindowAttributes {
        WindowAttributes {
            background_pixmap: 0,
            background_pixel: 0,
            border_pixmap: 0,
            border_pixel: 0,
            bit_gravity: BitGravity::Forget,
            win_gravity: WinGravity::Unmap,
            backing_store: BackingStore::NotUseful,
            backing_planes: 0,
            backing_pixel: 0,
            override_redirect: false,
            save_under: false,
            event_mask: 0,
            do_not_propagate_mask: 0,
            colormap: 0,
            cursor: 0,

            value_mask: 0,
        }
    }

    pub fn background_pixmap(&mut self, value: Pixmap) -> &mut Self {
        self.background_pixel = value;
        self.value_mask |= 0x00000001;
        self
    }

    pub fn background_pixel(&mut self, value: u32) -> &mut Self {
        self.background_pixel = value;
        self.value_mask |= 0x00000002;
        self
    }

    pub fn border_pixmap(&mut self, value: Pixmap) -> &mut Self {
        self.border_pixmap = value;
        self.value_mask |= 0x00000004;
        self
    }

    pub fn border_pixel(&mut self, value: u32) -> &mut Self {
        self.border_pixel = value;
        self.value_mask |= 0x00000008;
        self
    }

    pub fn bit_gravity(&mut self, value: BitGravity) -> &mut Self {
        self.bit_gravity = value;
        self.value_mask |= 0x00000010;
        self
    }

    pub fn win_gravity(&mut self, value: WinGravity) -> &mut Self {
        self.win_gravity = value;
        self.value_mask |= 0x00000020;
        self
    }

    pub fn backing_store(&mut self, value: BackingStore) -> &mut Self {
        self.backing_store = value;
        self.value_mask |= 0x00000040;
        self
    }

    pub fn backing_planes(&mut self, value: u32) -> &mut Self {
        self.backing_planes = value;
        self.value_mask |= 0x00000080;
        self
    }

    pub fn backing_pixel(&mut self, value: u32) -> &mut Self {
        self.backing_pixel = value;
        self.value_mask |= 0x00000100;
        self
    }

    pub fn override_redirect(&mut self, value: bool) -> &mut Self {
        self.override_redirect = value;
        self.value_mask |= 0x00000200;
        self
    }

    pub fn save_under(&mut self, value: bool) -> &mut Self {
        self.override_redirect = value;
        self.value_mask |= 0x00000400;
        self
    }

    pub fn event_mask(&mut self, value: Event) -> &mut Self {
        self.event_mask = value;
        self.value_mask |= 0x00000800;
        self
    }

    pub fn do_not_propagate_mask(&mut self, value: Event) -> &mut Self {
        self.do_not_propagate_mask = value;
        self.value_mask |= 0x00001000;
        self
    }

    pub fn colormap(&mut self, value: Colormap) -> &mut Self {
        self.do_not_propagate_mask = value;
        self.value_mask |= 0x00002000;
        self
    }

    pub fn cursor(&mut self, value: Cursor) -> &mut Self {
        self.cursor = value;
        self.value_mask |= 0x00004000;
        self
    }

    pub fn build(self) -> Self {
        self
    }

    fn encode(&self) -> io::Result<(Vec<u8>, u16)> {
        let mut count = 0u16;
        let mut a = io::Cursor::new(vec![]);

        a.write_u32::<NativeEndian>(self.value_mask)?;

        if (self.value_mask & 0x00000001) == 0x00000001 {
            a.write_u32::<NativeEndian>(self.background_pixmap)?;
            count += 1;
        }

        if (self.value_mask & 0x00000002) == 0x00000002 {
            a.write_u32::<NativeEndian>(self.background_pixel)?;
            count += 1;
        }

        if (self.value_mask & 0x00000004) == 0x00000004 {
            a.write_u32::<NativeEndian>(self.border_pixmap)?;
            count += 1;
        }

        if (self.value_mask & 0x00000008) == 0x00000008 {
            a.write_u32::<NativeEndian>(self.border_pixel)?;
            count += 1;
        }

        if (self.value_mask & 0x00000010) == 0x00000010 {
            a.write_u32::<NativeEndian>(self.bit_gravity.into())?;
            count += 1;
        }

        if (self.value_mask & 0x00000020) == 0x00000020 {
            a.write_u32::<NativeEndian>(self.win_gravity.into())?;
            count += 1;
        }

        if (self.value_mask & 0x00000040) == 0x00000040 {
            a.write_u32::<NativeEndian>(self.backing_store.into())?;
            count += 1;
        }

        if (self.value_mask & 0x00000080) == 0x00000080 {
            a.write_u32::<NativeEndian>(self.backing_planes)?;
            count += 1;
        }

        if (self.value_mask & 0x00000100) == 0x00000100 {
            a.write_u32::<NativeEndian>(self.backing_pixel)?;
            count += 1;
        }

        if (self.value_mask & 0x00000200) == 0x00000200 {
            a.write_u32::<NativeEndian>(if self.override_redirect {
                    0
                } else {
                    1
                })?;
            count += 1;
        }

        if (self.value_mask & 0x00000400) == 0x00000400 {
            a.write_u32::<NativeEndian>(if self.save_under {
                    0
                } else {
                    1
                })?;
            count += 1;
        }

        if (self.value_mask & 0x00000800) == 0x00000800 {
            a.write_u32::<NativeEndian>(self.event_mask)?;
            count += 1;
        }

        if (self.value_mask & 0x00001000) == 0x00001000 {
            a.write_u32::<NativeEndian>(self.do_not_propagate_mask)?;
            count += 1;
        }

        if (self.value_mask & 0x00002000) == 0x00002000 {
            a.write_u32::<NativeEndian>(self.colormap)?;
            count += 1;
        }

        if (self.value_mask & 0x00004000) == 0x00004000 {
            a.write_u32::<NativeEndian>(self.cursor)?;
            count += 1;
        }

        Ok((a.into_inner(), count))
    }
}
