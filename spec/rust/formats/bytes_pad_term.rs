// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct BytesPadTerm {
    pub str_pad: Vec<u8>,
    pub str_term: Vec<u8>,
    pub str_term_and_pad: Vec<u8>,
    pub str_term_include: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for BytesPadTerm {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.str_pad = _io.bytes_strip_right(_io.read_bytes(20 as usize)?, 64).to_vec();
        self.str_term = _io.bytes_terminate(_io.read_bytes(20 as usize)?, 64, false).to_vec();
        self.str_term_and_pad = _io.bytes_terminate(_io.bytes_strip_right(_io.read_bytes(20 as usize)?, 43), 64, false).to_vec();
        self.str_term_include = _io.bytes_terminate(_io.read_bytes(20 as usize)?, 64, true).to_vec();
        Ok(())
    }
}
impl<'r, 's: 'r> BytesPadTerm {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = BytesPadTerm::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
