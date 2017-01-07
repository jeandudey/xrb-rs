extern crate xrb;
extern crate futures;
extern crate tokio_core;

use tokio_core::reactor::Core;
use xrb::xc_misc::XCMiscGetXIDRange;
use xrb::Xauth;

fn main() {
    let path = Xauth::get_path().unwrap();
    let auth_info = Xauth::read_file(&path).unwrap();

    let mut lp = Core::new().unwrap();
    let req = xrb::Client::connect(1, &auth_info, lp.handle());

    let client = lp.run(req).unwrap();

    let req = client.perform_ex(XCMiscGetXIDRange);

    let (_, reply) = lp.run(req).unwrap();

    println!("XID Range: {:?}", reply);
}
