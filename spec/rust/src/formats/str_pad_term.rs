// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct StrPadTerm {
    pub str_pad: String,
    pub str_term: String,
    pub str_term_and_pad: String,
    pub str_term_include: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for StrPadTerm {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.str_pad = decode_string(_io.bytes_strip_right(_io.read_bytes(20 as usize)?, 64), "UTF-8")?;
        self.str_term = decode_string(_io.bytes_terminate(_io.read_bytes(20 as usize)?, 64, false), "UTF-8")?;
        self.str_term_and_pad = decode_string(_io.bytes_terminate(_io.bytes_strip_right(_io.read_bytes(20 as usize)?, 43), 64, false), "UTF-8")?;
        self.str_term_include = decode_string(_io.bytes_terminate(_io.read_bytes(20 as usize)?, 64, true), "UTF-8")?;
        Ok(())
    }
}
impl<'r, 's: 'r> StrPadTerm {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = StrPadTerm::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
