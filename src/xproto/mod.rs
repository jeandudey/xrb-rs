//! This module contains all X11 standard requests.

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

/// A window identifier.
pub type Window = u8;

pub type VisualId = u32;

pub type Pixmap = u32;

pub type Cursor = u32;

pub type Colormap = u32;

pub const NONE: u32 = 0;
pub const PARENT_RELATIVE: u32 = 1;

pub const COPY_FROM_PARENT: u16 = 0;
pub const INPUT_OUTPUT: u16 = 1;
pub const INPUT_ONLY: u16 = 2;

pub type BitGravity = u8;
pub type WinGravity = u8;

pub const GRAVITY_FORGET: u8 = 0;
pub const GRAVITY_UNMAP: u8 = 0;
pub const GRAVITY_NORTH_WEST: u8 = 1;
pub const GRAVITY_NORTH: u8 = 2;
pub const GRAVITY_NORTH_EAST: u8 = 3;
pub const GRAVITY_WEST: u8 = 4;
pub const GRAVITY_CENTER: u8 = 5;
pub const GRAVITY_EAST: u8 = 6;
pub const GRAVITY_SOUTH_WEST: u8 = 7;
pub const GRAVITY_SOUTH: u8 = 8;
pub const GRAVITY_SOUTH_EAST: u8 = 9;
pub const GRAVITY_STATIC: u8 = 10;

/// Re-exported `BackingStores`.
pub use super::BackingStores;

pub const EVENT_KEY_PRESS: u32 = 0x00000001;
pub const EVENT_KEY_RELEASE: u32 = 0x00000002;
pub const EVENT_BUTTON_PRESS: u32 = 0x00000004;
pub const EVENT_BUTTON_RELEASE: u32 = 0x00000008;
pub const EVENT_ENTER_WINDOW: u32 = 0x00000010;
pub const EVENT_LEAVE_WINDOW: u32 = 0x00000020;
pub const EVENT_POINTER_MOTION: u32 = 0x00000040;
pub const EVENT_POINTER_MOTION_HINT: u32 = 0x00000080;
pub const EVENT_BUTTON1_MOTION: u32 = 0x00000100;
pub const EVENT_BUTTON2_MOTION: u32 = 0x00000200;
pub const EVENT_BUTTON3_MOTION: u32 = 0x00000400;
pub const EVENT_BUTTON4_MOTION: u32 = 0x00000800;
pub const EVENT_BUTTON5_MOTION: u32 = 0x00001000;
pub const EVENT_BUTTON_MOTION: u32 = 0x00002000;
pub const EVENT_KEYMAP_STATE: u32 = 0x00004000;
pub const EVENT_EXPOSURE: u32 = 0x00008000;
pub const EVENT_VISIBILITY_CHANGE: u32 = 0x00010000;
pub const EVENT_STRUCTURE_NOTIFY: u32 = 0x00020000;
pub const EVENT_RESIZE_REDIRECT: u32 = 0x00040000;
pub const EVENT_SUBSTRUCTURE_NOTIFY: u32 = 0x00080000;
pub const EVENT_SUBSTRUCTURE_REDIRECT: u32 = 0x00100000;
pub const EVENT_FOCUS_CHANGE: u32 = 0x00200000;
pub const EVENT_PROPERTY_CHANGE: u32 = 0x00400000;
pub const EVENT_COLORMAP_CHANGE: u32 = 0x00800000;
pub const EVENT_OWNERGRAB_BUTTON: u32 = 0x01000000;
pub const EVENT_UNUSED: u32 = 0xFE000000;

pub const DEVICE_EVENT_BUTTON_PRESS: u32 = 0x00000004;
pub const DEVICE_EVENT_BUTTON_RELEASE: u32 = 0x00000008;
pub const DEVICE_EVENT_ENTER_WINDOW: u32 = 0x00000010;
pub const DEVICE_EVENT_LEAVE_WINDOW: u32 = 0x00000020;
pub const DEVICE_EVENT_POINTER_MOTION: u32 = 0x00000040;
pub const DEVICE_EVENT_POINTER_MOTION_HINT: u32 = 0x00000080;
pub const DEVICE_EVENT_BUTTON1_MOTION: u32 = 0x00000100;
pub const DEVICE_EVENT_BUTTON2_MOTION: u32 = 0x00000200;
pub const DEVICE_EVENT_BUTTON3_MOTION: u32 = 0x00000400;
pub const DEVICE_EVENT_BUTTON4_MOTION: u32 = 0x00000800;
pub const DEVICE_EVENT_BUTTON5_MOTION: u32 = 0x00001000;
pub const DEVICE_EVENT_BUTTON_MOTION: u32 = 0x00002000;
pub const DEVICE_EVENT_KEYMAP_STATE: u32 = 0x00004000;
pub const DEVICE_EVENT_UNUSED: u32 = 0xFFFF8003;
