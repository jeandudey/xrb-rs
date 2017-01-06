extern crate xrb;
extern crate tokio_core;
extern crate futures;

use tokio_core::reactor::Core;
use xrb::xproto::ListExtensions;
use xrb::Xauth;

fn main() {
    let path = Xauth::get_path().unwrap();
    let auth_info = Xauth::read_file(&path).unwrap();

    let mut lp = Core::new().unwrap();
    let req = xrb::Client::connect(1, &auth_info, lp.handle());

    let client = lp.run(req).unwrap();

    let req = client.perform(ListExtensions);
    let (_, list) = lp.run(req).unwrap();

    println!("Aviable extensions:");

    for extension_name in list {
        println!("{}", extension_name);
    }
}
