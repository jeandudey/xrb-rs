extern crate xrb;
extern crate tokio_core;
extern crate xauth;

use tokio_core::reactor::Core;

#[test]
fn connect_uds_with_display() {
    let lp = Core::new().unwrap();
    xrb::connect(0, lp.handle()).unwrap();
}

#[test]
fn connect_uds_without_display() {
    let lp = Core::new().unwrap();
    xrb::connect(None, lp.handle()).unwrap();
}

#[test]
fn setup_connection() {
    use xrb::setup::Setup;
    use xauth::Xauth;

    let mut lp = Core::new().unwrap();
    let socket = xrb::connect(None, lp.handle()).unwrap();

    // get auth info
    let path = Xauth::get_path().unwrap(); 
    let auth_info = Xauth::read_file(&path).unwrap();

    let req = xrb::setup(socket, &auth_info);
    let res = lp.run(req).unwrap();

    match res.1 {
        Setup::Success { header } => panic!("header: {:?}", header),
        Setup::Failed { reason, header } => panic!("Setup failed: {} {:?}", reason, header),
        Setup::Authenticate { .. } => panic!("Setup authenticate"),
    }
}
