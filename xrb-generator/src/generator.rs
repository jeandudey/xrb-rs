use ::std;
use std::io;
use std::io::BufWriter;
use std::io::Write;
use std::fs::File;
use std::path::Path;

use ::proto;
use proto::Fields;

pub fn generate<P: AsRef<Path>>(output_file: P, root: ::proto::Xcb) -> io::Result<()> {
    let mut f = File::create(output_file).map(BufWriter::new)?;

    for struc in root.structs {
        writeln!(&mut f, "pub struct {} {{", struc.name)?;
        for field in struc.fields {
            match field {
                Fields::Field(fi) => {
                    writeln!(&mut f,
                             "    pub {}: {},",
                             fi.name,
                             to_rust_type(fi.type_.as_str()))
                        ?
                }
                _ => unreachable!(),
            }
        }
        writeln!(&mut f, "}}")?;
    }

    f.flush()?;

    Ok(())
}

fn to_rust_type(ty: &str) -> &'static str {
    match ty {
        "CARD8" => "u8",
        "CARD16" => "u16",
        "CARD32" => "u32",
        "INT8" => "i8",
        "INT16" => "i16",
        "INT32" => "i32",
        "BOOL" => "bool",
        "ATOM" => "u32",
        "SURFACE" => "u32",
        _ => panic!("Not a valid type: {}", ty),
    }
}
