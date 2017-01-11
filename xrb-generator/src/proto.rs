//! Contains code for parsing `xcb-proto` files.

use ::std;
use std::fmt;
use std::fmt::Display;
use std::io::Read;
use std::str::FromStr;
use std::num::ParseIntError;

use ::xml;
use xml::reader::EventReader;
use xml::reader::XmlEvent;
use xml::attribute::OwnedAttribute;

pub fn parse<R: Read>(reader: &mut R) -> Result<Xcb, Error> {
    enum State {
        Start,
        End,
        Xcb,
    }

    let mut state = State::Start;
    let mut xcb_tag = Xcb::default();
    let event_reader = EventReader::new(reader);

    for e in event_reader {
        let e = try!(e);

        state = match state {
            State::Start => {
                match e {
                    XmlEvent::StartDocument { .. } => State::Start,
                    XmlEvent::StartElement { ref name, ref attributes, .. } => {
                        if &*name.local_name == "xcb" {
                            try!(xcb_tag.parse_attributes(attributes));
                            State::Xcb
                        } else {
                            return Err(Error::from("Invalid start element"));
                        }
                    }
                    _ => {
                        return Err(Error::from("Expected XmlEvent::StartDocument or \
                                                XmlEvent::StartElement"))
                    }
                }
            }
            State::End => {
                match e {
                    XmlEvent::EndDocument => return Ok(xcb_tag),
                    _ => return Err(Error::from("Expected EOF")),
                }
            }
            State::Xcb => State::Xcb,
        };
    }

    Ok(xcb_tag)
}

/// A parsing error
#[derive(Debug)]
pub enum Error {
    /// Error contained in a string.
    StringError(String),

    /// Error when parsing integer.
    ParseInt(ParseIntError),

    /// XML Parsing error.
    Xml(xml::reader::Error),
}

impl From<String> for Error {
    fn from(e: String) -> Error {
        Error::StringError(e)
    }
}

impl From<&'static str> for Error {
    fn from(e: &'static str) -> Error {
        Error::StringError(e.to_string())
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Error {
        Error::ParseInt(e)
    }
}

impl From<xml::reader::Error> for Error {
    fn from(e: xml::reader::Error) -> Error {
        Error::Xml(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::StringError(ref e) => write!(f, "{}", e),
            Error::ParseInt(ref e) => write!(f, "Integer parsing error: {}", e),
            Error::Xml(ref e) => write!(f, "XML Parsing error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::StringError(_) => "Parsing error",
            Error::ParseInt(_) => "Integer parsing error",
            Error::Xml(_) => "XML Parsing error",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::ParseInt(ref e) => Some(e),
            Error::Xml(ref e) => Some(e),
            _ => None,
        }
    }
}

/// Xcb Tag
#[derive(Debug, Default)]
pub struct Xcb {
    header: String,
    extension_xname: Option<String>,
    extension_name: Option<String>,
    extension_multiword: Option<bool>,
    major_version: Option<usize>,
    minor_version: Option<usize>,
}

impl Xcb {
    fn parse_attributes(&mut self, attributes: &[OwnedAttribute]) -> Result<(), Error> {
        for attr in attributes {
            match &*attr.name.local_name {
                "header" => self.header = attr.value.clone(),
                "extension-xname" => self.extension_xname = Some(attr.value.clone()),
                "extension-name" => self.extension_name = Some(attr.value.clone()),
                "extension-multiword" => {
                    self.extension_multiword = match &*attr.value {
                        "true" | "1" => Some(true),
                        "false" | "0" => Some(false),
                        _ => return Err(Error::from("Not valid boolean")),
                    }
                }
                "major-version" => {
                    self.major_version = Some(try!(usize::from_str(attr.value
                        .as_str())));
                }
                "minor-version" => {
                    self.minor_version = Some(try!(usize::from_str(attr.value
                        .as_str())));
                }
                _ => return Err(Error::from("Invalid attribute for <xcb> tag")),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use ::std;
    use std::io;

    #[test]
    fn minimal_xcb_proto_file() {
        let s = r#"<?xml version="1.0" encoding="utf-8"?>
                   <xcb header="xproto">
                   </xcb>"#;

        let mut reader = io::Cursor::new(s);
        let root = super::parse(&mut reader).unwrap();
        assert_eq!(root.header, "xproto");
        assert_eq!(root.extension_xname, None);
        assert_eq!(root.extension_name, None);
        assert_eq!(root.extension_multiword, None);
        assert_eq!(root.major_version, None);
        assert_eq!(root.minor_version, None);
    }
}
