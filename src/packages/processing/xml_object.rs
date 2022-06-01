use super::ProcessingError;

use std::io::{self, BufRead, BufReader, Read};

use fast_xml::Reader;

pub trait XMLObject
where
    Self: Sized,
{
    fn from_xml_bytes(bytes: Vec<u8>) -> Result<Self, ProcessingError> {
        return Self::from_xml_bufread(io::Cursor::new(bytes));
    }

    fn from_xml_str(s: &str) -> Result<Self, ProcessingError>;

    fn from_xml_read<R: Read>(reader: R) -> Result<Self, ProcessingError> {
        return Self::from_xml_bufread(BufReader::new(reader));
    }

    fn from_xml_bufread<R: BufRead>(reader: R) -> Result<Self, ProcessingError>;
}

impl<T> XMLObject for T
where
    T: XMLManualObject,
{
    fn from_xml_str(s: &str) -> Result<Self, ProcessingError> {
        return Self::from_xml_reader(Reader::from_str(s));
    }

    fn from_xml_read<R: Read>(reader: R) -> Result<Self, ProcessingError> {
        return Self::from_xml_bufread(BufReader::new(reader));
    }

    fn from_xml_bufread<R: BufRead>(reader: R) -> Result<Self, ProcessingError> {
        return Self::from_xml_reader(Reader::from_reader(reader));
    }
}

pub trait XMLManualObject
where
    Self: Sized,
{
    fn from_xml_reader<B: BufRead>(reader: Reader<B>) -> Result<Self, ProcessingError>;
}
