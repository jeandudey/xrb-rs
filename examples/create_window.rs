extern crate xrb;
extern crate xauth;
extern crate tokio_core;

use xauth::Xauth;
use tokio_core::reactor::Core;
use xrb::Setup;
use xrb::xproto::{CreateWindow, MapWindow};

fn main() {
    let mut lp = Core::new().unwrap();
    let socket = xrb::connect(0, lp.handle()).unwrap();

    let path = Xauth::get_path().unwrap();
    let auth_info = Xauth::read_file(&path).unwrap();

    // setup connection and get server information as response
    let req = xrb::setup(socket, &auth_info);
    let (socket, res) = lp.run(req).unwrap();

    let server_info = match res {
        Setup::Success(s) => s,
        _ => panic!("Couldn't connect to host"),
    };

    let wid = server_info.resource_id_base + 1;
    let parent = server_info.roots[0].root;

    let req = CreateWindow::new(wid, parent, 1, 24, 0, 100, 100, 1024, 1024, 0).perform(socket);
    let (socket, _) = lp.run(req).unwrap();


    let req = MapWindow::new(wid).perform(socket);
    let (socket, _) = lp.run(req).unwrap();

    loop {
    }
}
