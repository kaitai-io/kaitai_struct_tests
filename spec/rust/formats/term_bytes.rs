// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct TermBytes {
    pub s1: Vec<u8>,
    pub s2: Vec<u8>,
    pub s3: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for TermBytes {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.s1 = _io.read_bytes_term(124, false, true, true)?;
        self.s2 = _io.read_bytes_term(124, false, false, true)?;
        self.s3 = _io.read_bytes_term(64, true, true, true)?;
        Ok(())
    }
}
impl<'r, 's: 'r> TermBytes {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = TermBytes::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
