extern crate xrb;
extern crate tokio_core;
extern crate futures;

use tokio_core::reactor::Core;
use xrb::xproto::QueryExtension;
use xrb::Xauth;

fn main() {
    let path = Xauth::get_path().unwrap();
    let auth_info = Xauth::read_file(&path).unwrap();

    let mut lp = Core::new().unwrap();
    let req = xrb::Client::connect(1, &auth_info, lp.handle());

    let client = lp.run(req).unwrap();

    let req = client.perform(QueryExtension { name: b"XVideo".to_vec() });

    let (_, ex_info) = lp.run(req).unwrap();

    println!("XVideo Extension Info:\n{:?}", ex_info);
}
