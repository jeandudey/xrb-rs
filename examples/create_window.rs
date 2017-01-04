extern crate xrb;
extern crate xauth;
extern crate tokio_core;
extern crate futures;

use xauth::Xauth;
use tokio_core::reactor::Core;
use xrb::xproto::{CreateWindow, MapWindow};
use futures::Future;

fn main() {

    let path = Xauth::get_path().unwrap();
    let auth_info = Xauth::read_file(&path).unwrap();

    let mut lp = Core::new().unwrap();
    let req = xrb::Client::connect(1, &auth_info, lp.handle());

    let client = lp.run(req).unwrap();

    let (wid, parent) = {
        let server_info = client.get_server_info();
        let wid = server_info.resource_id_base + 1;
        let parent = server_info.roots[0].root;
        (wid, parent)
    };

    let req = CreateWindow::new(wid, parent, 1, 24, 0, 100, 100, 200, 200, 0).perform(client);
    let (client, _) = lp.run(req).unwrap();

    let req = MapWindow::new(wid).perform(client);
    let (client, _) = lp.run(req).unwrap();

    println!("going loop");;

    loop {}
}
