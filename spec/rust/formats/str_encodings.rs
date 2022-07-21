// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct StrEncodings {
    pub len_of_1: u16,
    pub str1: String,
    pub len_of_2: u16,
    pub str2: String,
    pub len_of_3: u16,
    pub str3: String,
    pub len_of_4: u16,
    pub str4: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for StrEncodings {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.len_of_1 = _io.read_u2le()?;
        self.str1 = decode_string(_io.read_bytes(self.len_of_1 as usize)?, "ASCII")?;
        self.len_of_2 = _io.read_u2le()?;
        self.str2 = decode_string(_io.read_bytes(self.len_of_2 as usize)?, "UTF-8")?;
        self.len_of_3 = _io.read_u2le()?;
        self.str3 = decode_string(_io.read_bytes(self.len_of_3 as usize)?, "SJIS")?;
        self.len_of_4 = _io.read_u2le()?;
        self.str4 = decode_string(_io.read_bytes(self.len_of_4 as usize)?, "CP437")?;
        Ok(())
    }
}
impl<'r, 's: 'r> StrEncodings {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = StrEncodings::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
