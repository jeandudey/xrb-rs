extern crate xml;

mod proto;

use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::env;

fn main() {
    let input_folder = match env::args().nth(1) {
        Some(f) => f,
        None => {
            println!("Usage:");
            println!("\txcb-genertor INPUT_FOLDER OUTPUT_FOLDER");
            return;
        }
    };

    let output_folder = match env::args().nth(2) {
        Some(f) => f,
        None => {
            println!("Usage:");
            println!("\txcb-genertor INPUT_FOLDER OUTPUT_FOLDER");
            return;
        }
    };

    let read_dir = match fs::read_dir(input_folder) {
        Ok(r) => r,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    for entry in read_dir {
        let path = entry.unwrap().path();
        if let Some(e) = path.extension() {
            if e == "xsd" {
                continue;
            } else if e != "xml" {
                println!("Warning: Invalid file on folder: {}", path.display());
                continue;
            }
        } else {
            println!("Warning: Invalid file on folder: {}", path.display());
            continue;
        }

        let mut file = match File::open(path.clone()) {
            Ok(f) => f,
            Err(e) => {
                println!("Error: {}: {}", path.display(), e);
                continue;
            }
        };

        let root: proto::Xcb = match proto::parse(&mut file) {
            Ok(x) => x,
            Err(e) => {
                println!("Error: {}: {}", path.display(), e);
                continue;
            }
        };

        println!("{:?}", root);
    }
}
