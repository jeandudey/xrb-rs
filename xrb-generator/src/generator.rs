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
                Fields::Pad(pad) => (),
            }
        }
        writeln!(&mut f, "}}\n")?;
    }

    f.flush()?;

    Ok(())
}

fn to_rust_type(ty: &str) -> String {
    use std::ops::Index;

    match ty {
        "CARD8" => "u8".to_string(),
        "CARD16" => "u16".to_string(),
        "CARD32" => "u32".to_string(),
        "INT8" => "i8".to_string(),
        "INT16" => "i16".to_string(),
        "INT32" => "i32".to_string(),
        "BOOL" => "bool".to_string(),
        _ => {
            let split = ty.split_at(1);
            let mut lower = split.1.to_lowercase();
            lower.insert(0, split.0.chars().nth(0).unwrap());
            lower
        }
    }
}

#[cfg(test)]
mod tests {
    fn to_rust_type() {
        let rty = super::to_rust_type("DOTCLOCK");
        assert_eq!(rty, "Dotclock");

        let rty = super::to_rust_type("CARD8");
        assert_eq!(rty, "u8");
    }
}
