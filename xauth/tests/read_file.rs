extern crate xauth;

use xauth::Xauth;

#[test]
fn read_file() {
    let path = Xauth::get_path().unwrap(); 
    Xauth::read_file(&path).unwrap();
}
