extern crate xrb;
extern crate futures;
extern crate tokio_core;

use tokio_core::reactor::Core;
use xrb::xc_misc::XCMiscGetVersion;
use xrb::Xauth;

fn main() {
    let path = Xauth::get_path().unwrap();
    let auth_info = Xauth::read_file(&path).unwrap();

    let mut lp = Core::new().unwrap();
    let req = xrb::Client::connect(0, &auth_info, lp.handle());

    let client = lp.run(req).unwrap();

    let req = client.perform_ex(XCMiscGetVersion {
        client_major_version: 11,
        client_minor_version: 0,
    });

    let (_, reply) = lp.run(req).unwrap();

    println!("XC-MISC Supported version:\n{:?}", reply);
}
