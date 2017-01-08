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

/// A X11 Protocol ID
pub type Xid = u32;

/// A window identifier.
pub type Window = Xid;

/// Re-exported `BackingStores`.
pub use super::BackingStores;

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
