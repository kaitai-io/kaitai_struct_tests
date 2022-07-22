// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ParamsDef {
    pub len: u32,
    pub has_trailer: bool,
    pub buf: String,
    pub trailer: u8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ParamsDef {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.buf = decode_string(_io.read_bytes(self.len as usize)?, "UTF-8")?;
        {
            // condIfHeader(Name(identifier(has_trailer)))
            self.trailer = _io.read_u1()?;
        }
        Ok(())
    }
}
impl<'r, 's: 'r> ParamsDef {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ParamsDef::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
