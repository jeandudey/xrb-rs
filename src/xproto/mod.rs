//! This module contains all X11 standard requests.

mod create_window;
pub use self::create_window::*;

mod map_window;
pub use self::map_window::*;

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

/// Re-exported BackingStores.
pub use super::BackingStores;

bitflags! {
    flags Event: u32 {
        const EVENT_KEY_PRESS = 0x00000001,
        const EVENT_KEY_RELEASE = 0x00000002,
        const EVENT_BUTTON_PRESS = 0x00000004,
        const EVENT_BUTTON_RELEASE = 0x00000008,
        const EVENT_ENTER_WINDOW = 0x00000010,
        const EVENT_LEAVE_WINDOW = 0x00000020,
        const EVENT_POINTER_MOTION = 0x00000040,
        const EVENT_POINTER_MOTION_HINT = 0x00000080,
        const EVENT_BUTTON1_MOTION = 0x00000100,
        const EVENT_BUTTON2_MOTION = 0x00000200,
        const EVENT_BUTTON3_MOTION = 0x00000400,
        const EVENT_BUTTON4_MOTION = 0x00000800,
        const EVENT_BUTTON5_MOTION = 0x00001000,
        const EVENT_BUTTON_MOTION = 0x00002000,
        const EVENT_KEYMAP_STATE = 0x00004000,
        const EVENT_EXPOSURE = 0x00008000,
        const EVENT_VISIBILITY_CHANGE = 0x00010000,
        const EVENT_STRUCTURE_NOTIFY = 0x00020000,
        const EVENT_RESIZE_REDIRECT  = 0x00040000,
        const EVENT_SUBSTRUCTURE_NOTIFY = 0x00080000,
        const EVENT_SUBSTRUCTURE_REDIRECT = 0x00100000,
        const EVENT_FOCUS_CHANGE = 0x00200000,
        const EVENT_PROPERTY_CHANGE = 0x00400000,
        const EVENT_COLORMAP_CHANGE = 0x00800000,
        const EVENT_OWNERGRAB_BUTTON = 0x01000000,
        const EVENT_UNUSED = 0xFE000000,
    }
}

bitflags! {
    flags DeviceEvent: u32 {
        const DEVICE_EVENT_BUTTON_PRESS = 0x00000004,
        const DEVICE_EVENT_BUTTON_RELEASE = 0x00000008,
        const DEVICE_EVENT_ENTER_WINDOW = 0x00000010,
        const DEVICE_EVENT_LEAVE_WINDOW = 0x00000020,
        const DEVICE_EVENT_POINTER_MOTION = 0x00000040,
        const DEVICE_EVENT_POINTER_MOTION_HINT = 0x00000080,
        const DEVICE_EVENT_BUTTON1_MOTION = 0x00000100,
        const DEVICE_EVENT_BUTTON2_MOTION = 0x00000200,
        const DEVICE_EVENT_BUTTON3_MOTION = 0x00000400,
        const DEVICE_EVENT_BUTTON4_MOTION = 0x00000800,
        const DEVICE_EVENT_BUTTON5_MOTION = 0x00001000,
        const DEVICE_EVENT_BUTTON_MOTION = 0x00002000,
        const DEVICE_EVENT_KEYMAP_STATE = 0x00004000,
        const DEVICE_EVENT_UNUSED = 0xFFFF8003,
    }
}
