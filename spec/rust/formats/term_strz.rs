// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct TermStrz {
    pub s1: String,
    pub s2: String,
    pub s3: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for TermStrz {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.s1 = decode_string(_io.read_bytes_term(124, false, true, true)?, "UTF-8")?;
        self.s2 = decode_string(_io.read_bytes_term(124, false, false, true)?, "UTF-8")?;
        self.s3 = decode_string(_io.read_bytes_term(64, true, true, true)?, "UTF-8")?;
        Ok(())
    }
}
impl<'r, 's: 'r> TermStrz {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = TermStrz::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
