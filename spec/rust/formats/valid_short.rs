// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ValidShort {
    pub magic1: Vec<u8>,
    pub uint8: u8,
    pub sint8: i8,
    pub magic_uint: String,
    pub uint16: u16,
    pub uint32: u32,
    pub uint64: u64,
    pub magic_sint: String,
    pub sint16: i16,
    pub sint32: i32,
    pub sint64: i64,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ValidShort {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.magic1 = _io.read_bytes(6 as usize)?.to_vec();
        self.uint8 = _io.read_u1()?;
        self.sint8 = _io.read_s1()?;
        self.magic_uint = decode_string(_io.read_bytes(10 as usize)?, "utf-8")?;
        self.uint16 = _io.read_u2le()?;
        self.uint32 = _io.read_u4le()?;
        self.uint64 = _io.read_u8le()?;
        self.magic_sint = decode_string(_io.read_bytes(10 as usize)?, "utf-8")?;
        self.sint16 = _io.read_s2le()?;
        self.sint32 = _io.read_s4le()?;
        self.sint64 = _io.read_s8le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> ValidShort {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ValidShort::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
