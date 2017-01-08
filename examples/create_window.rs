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
