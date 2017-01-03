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

#[macro_use]
pub mod macros;
pub mod utils;

use tokio_core::reactor::Handle;
use tokio_uds::UnixStream;
use std::io::{self, Read, Write};
use futures::Future;
use byteorder::{WriteBytesExt, ReadBytesExt, NativeEndian};
use utils::pad;
use xauth::Xauth;

/// Connects X11 server using unix domain sockets at the specified display.
/// If display is `None` then the display specified on `DISPLAY` environment
/// variable will be used.
///
/// The returned stream will be used to communicate to the X11 server.
///
/// # Panics
/// A panic will be thrown if you pass `None` as display and the `DISPLAY`
/// environment variable is not found.
pub fn connect<D>(display: D, handle: Handle) -> io::Result<UnixStream>
    where D: Into<Option<u16>>
{
    let disp = if let Some(d) = display.into() {
        d
    } else {
        utils::get_default_display_number().expect("Failed to get DISPLAY environment variable")
    };

    let path = format!("/tmp/.X11-unix/X{}", disp);
    UnixStream::connect(&path, &handle)
}

mod setup;
pub use setup::*;

pub mod xproto;
