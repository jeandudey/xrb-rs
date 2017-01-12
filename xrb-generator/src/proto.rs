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
        Struct(Struct),
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
            State::Xcb => {
                match e {
                    XmlEvent::StartElement { ref name, ref attributes, .. } => {
                        match &*name.local_name {
                            "struct" => {
                                let mut s = Struct::default();
                                try!(s.parse_attributes(attributes));
                                State::Struct(s)
                            }
                            _ => State::Xcb,
                        }
                    }
                    XmlEvent::EndElement { .. } => State::Xcb,
                    _ => State::Xcb,
                }
            }
            State::Struct(mut s) => {
                match e {
                    XmlEvent::StartElement { ref name, ref attributes, .. } => {
                        match &*name.local_name {
                            "field" => {
                                let mut field = Field::default();
                                field.parse_attributes(attributes);
                                s.fields.push(Fields::Field(field));
                            }
                            _ => return Err(Error::from("Invalid <struct> tag child")),
                        }

                        State::Struct(s)
                    }
                    XmlEvent::EndElement { ref name, .. } => {
                        match &*name.local_name {
                            "struct" => {
                                xcb_tag.structs.push(s);
                                State::Xcb
                            }
                            _ => State::Struct(s),
                        }
                    }
                    _ => State::Struct(s),
                }
            }
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
    // Attributes
    pub header: String,
    pub extension_xname: Option<String>,
    pub extension_name: Option<String>,
    pub extension_multiword: Option<bool>,
    pub major_version: Option<usize>,
    pub minor_version: Option<usize>,

    // Childs
    pub structs: Vec<Struct>,
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

#[derive(Debug, Default)]
pub struct Struct {
    // Attributes
    pub name: String,

    // Childs
    pub fields: Vec<Fields>,
}

impl Struct {
    fn parse_attributes(&mut self, attributes: &[OwnedAttribute]) -> Result<(), Error> {
        for attr in attributes {
            match &*attr.name.local_name {
                "name" => self.name = attr.value.clone(),
                _ => return Err(Error::from("Invalid attribute for <struct> tag")),
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum Fields {
    Pad(usize),
    Field(Field),
}

#[derive(Debug, Default)]
pub struct Field {
    // Attributes
    pub name: String,
    pub type_: String,
    pub enum_: Option<String>,
    pub altenum: Option<String>,
    pub mask: Option<String>,
}

impl Field {
    fn parse_attributes(&mut self, attributes: &[OwnedAttribute]) -> Result<(), Error> {
        for attr in attributes {
            match &*attr.name.local_name {
                "name" => self.name = attr.value.clone(),
                "type" => self.type_ = attr.value.clone(),
                "enum" => self.enum_ = Some(attr.value.clone()),
                "altenum" => self.altenum = Some(attr.value.clone()),
                "mask" => self.mask = Some(attr.value.clone()),
                _ => return Err(Error::from("Invalid attribute for <field> tag")),
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

    #[test]
    fn parse_struct() {
        use super::Fields;

        let s = r#"<?xml version="1.0" encoding="utf-8"?>
                   <xcb header="xproto">
                     <struct name="CHAR2B">
                       <field type="CARD8" name="byte1" />
                       <field type="CARD8" name="byte2" />
                     </struct>
                   </xcb>"#;

        let mut reader = io::Cursor::new(s);
        let root = super::parse(&mut reader).unwrap();
        assert_eq!(root.structs[0].name, "CHAR2B");
        match root.structs[0].fields[0] {
            Fields::Field(ref f) => assert_eq!(f.name, "byte1"),
            _ => panic!("Not valid field"),
        }

        match root.structs[0].fields[1] {
            Fields::Field(ref f) => assert_eq!(f.name, "byte2"),
            _ => panic!("Not valid field"),
        }
    }
}
