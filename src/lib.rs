/// xrb-rs - X Rust Bindings
///
/// This crate is a proof of concept, it's not ready for production yet, but
/// in the future this crate aims to be a good alternative to Xlib and XCB.
///
/// # Usage
/// As this crate isn't published on crates.io for obvious reasons you need
/// to add it to cargo as a git repository.
///
/// ```toml
/// [dependencies]
/// xrb = { git = "https://www.github.com/jeandudey/xrb-rs/" }
/// ```
///
/// And put this obvious code to your crate root.
///
/// ```rust
/// extern crate xrb;
/// ```
///
/// Now you can start using it or, **de facto**, debugging it.
///
/// # Connecting
/// The only supported method of connection is trough Unix Domain Sockets,
/// because *TCP* is *mainstream* and a bunch of **geeks** (nerds) don't
/// want to use it because they say it's unsafe.

extern crate tokio_core;
extern crate tokio_uds;
extern crate futures;
extern crate byteorder;
extern crate xauth;

use tokio_core::reactor::Handle;
use tokio_uds::UnixStream;
use std::io;

#[macro_use]
pub mod macros;

/// Connects X11 server using unix domain sockets at the specified display.
/// If display is `None` then the display specified on `DISPLAY` environment
/// variable will be used.
///
/// The returned stream will be used to communicate to the X11 server.
///
/// # Panics
/// A panic will be thrown if you pass `None` as display and the DISPLAY
/// environment variable
pub fn connect<D>(display: D, handle: Handle) -> io::Result<UnixStream> where D: Into<Option<u16>>{
    let disp = if let Some(d) = display.into() {
        d
    } else {
        utils::get_default_display_number().expect("Failed to get DISPLAY environment variable")
    };

    let path = format!("/tmp/.X11-unix/X{}", disp);
    UnixStream::connect(&path, &handle)
}

pub mod setup;
pub use setup::setup;

pub mod utils;
