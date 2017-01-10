xrb-rs - Rust Powered X11 Client using Futures.
-----------------------------------------------

[![Build Status](https://travis-ci.org/jeandudey/xrb-rs.svg?branch=master)](https://travis-ci.org/jeandudey/xrb-rs)

This projects aim to be the best client for communicating with X11 server effectively, using native *Rust* code! The project is still in an early development stage and not production ready, but in the end of the year is planned to be released a 1.0 stable version.

# Why Rust?
Rust is a safe system programming language designed to be fast but preserving features of modern programming languages with no runtime cost. Also pleople love it and it's gaining a lot of attention.

# Getting Started

## Prerequisites
You need a Rust compiler (>=1.12) and a X11 server to work with.

## Installation
Put the following in your `Cargo.toml`:

```toml
[dependencies.xrb]
git = "https://github.com/jeandudey/xrb-rs"
```

And this to your crate root:
```rust
extern crate xrb;
extern crate tokio_core;
extern crate futures;
```


## Example
This example shows how to create a window.

```rust
extern crate xrb;
extern crate tokio_core;
extern crate futures;

use tokio_core::reactor::Core;
use xrb::xproto::CreateWindow;
use xrb::xproto::MapWindow;
use xrb::Xauth;
use futures::Future;

fn main() {

    let path = Xauth::get_path().unwrap();
    let auth_info = Xauth::read_file(&path).unwrap();

    let mut lp = Core::new().unwrap();
    let req = xrb::Client::connect(1, &auth_info, lp.handle());

    let client = lp.run(req).unwrap();

    let parent = client.get_server_info().roots[0].root;

    let req = client.generate_id().and_then(|(client, id)| {
        client.perform(CreateWindow {
                wid: id,
                parent: parent,
                class: 1,
                depth: 24,
                visual: 0,
                x: 100,
                y: 100,
                width: 200,
                height: 200,
                border_width: 0,
            })
            .and_then(move |(client, _)| client.perform(MapWindow { wid: id }))
    });

    let (client, _) = lp.run(req).unwrap();

    println!("going loop");

    loop {}
}
```

# Contribuiting
All contributions are appreciated! Here are some basic instructions:
- Fork and clone the project.
- Create a topic branch (example: feat/resolve-protcol-error)
- Commit your changes using the projects commit guidelines.
- Squash your commits (if necessary).
- Make sure your code compiles.
- Open a pull request.
- Wait for a review.

**Note:** if your change affect some code that it's exposed on README.md please update it.

# Authors
- **Jean Pierre Dudey** - *Initial work* - [jeandudey][my-profile]

# Donations
I you wan't to see this project evolve please consider donating, i will appreciate it!

**BitCoin**: `144VTHr1vCmFvtjnv1ThMXuNA1SDDjxK3h`

**LiteCoin**: `LL3jvLLAp1oq6mshu9DCiMUYAMnsZeqra2`

**Flattr**: https://flattr.com/profile/jeandudey

# Self-Promotion
If you like **xrb** - give it a star and share it.

# License
`xrb-rs` is licensed under the terms of the MIT license. You can find a full copy of it [here][license].

# Contact
My email: jeandudey@hotmail.com

# Acknowledgements
This project was't made possible without:
- [futures-rs][1]
- [tokio-core][2]
- [byteorder][3]
- [The X.Org X11 Protocol Standard][4]

[1]: https://github.com/alexcrichton/futures-rs/
[2]: https://github.com/tokio-rs/tokio-core/
[3]: https://github.com/BurntSushi/byteorder/
[4]: http://www.x.org/releases/X11R7.7/doc/xproto/x11protocol.html
[my-profile]: https://github.com/jeandudey/
[license]:  https://github.com/jeandudey/xrb-rs/blob/master/LICENSE
