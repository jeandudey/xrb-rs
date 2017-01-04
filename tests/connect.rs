extern crate xrb;
extern crate tokio_core;
extern crate xauth;

use tokio_core::reactor::Core;
use xauth::Xauth;

#[test]
fn setup_connection() {
    let path = Xauth::get_path().unwrap();
    let auth_info = Xauth::read_file(&path).unwrap();

    let mut lp = Core::new().unwrap();
    let req = xrb::Client::connect(1, &auth_info, lp.handle());

    lp.run(req).unwrap();
}
