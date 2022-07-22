// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ValidNotParsedIf {
    pub not_parsed: u8,
    pub parsed: u8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ValidNotParsedIf {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        {
            // condIfHeader(Bool(false))
            self.not_parsed = _io.read_u1()?;
        }
        {
            // condIfHeader(Bool(true))
            self.parsed = _io.read_u1()?;
        }
        Ok(())
    }
}
impl<'r, 's: 'r> ValidNotParsedIf {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ValidNotParsedIf::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
