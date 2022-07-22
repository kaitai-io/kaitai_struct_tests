// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct Integers {
    pub magic1: Vec<u8>,
    pub uint8: u8,
    pub sint8: i8,
    pub magic_uint: Vec<u8>,
    pub uint16: u16,
    pub uint32: u32,
    pub uint64: u64,
    pub magic_sint: Vec<u8>,
    pub sint16: i16,
    pub sint32: i32,
    pub sint64: i64,
    pub magic_uint_le: Vec<u8>,
    pub uint16le: u16,
    pub uint32le: u32,
    pub uint64le: u64,
    pub magic_sint_le: Vec<u8>,
    pub sint16le: i16,
    pub sint32le: i32,
    pub sint64le: i64,
    pub magic_uint_be: Vec<u8>,
    pub uint16be: u16,
    pub uint32be: u32,
    pub uint64be: u64,
    pub magic_sint_be: Vec<u8>,
    pub sint16be: i16,
    pub sint32be: i32,
    pub sint64be: i64,
}
impl<'r, 's: 'r> KStruct<'r, 's> for Integers {
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
        self.magic_uint = _io.read_bytes(10 as usize)?.to_vec();
        self.uint16 = _io.read_u2le()?;
        self.uint32 = _io.read_u4le()?;
        self.uint64 = _io.read_u8le()?;
        self.magic_sint = _io.read_bytes(10 as usize)?.to_vec();
        self.sint16 = _io.read_s2le()?;
        self.sint32 = _io.read_s4le()?;
        self.sint64 = _io.read_s8le()?;
        self.magic_uint_le = _io.read_bytes(9 as usize)?.to_vec();
        self.uint16le = _io.read_u2le()?;
        self.uint32le = _io.read_u4le()?;
        self.uint64le = _io.read_u8le()?;
        self.magic_sint_le = _io.read_bytes(9 as usize)?.to_vec();
        self.sint16le = _io.read_s2le()?;
        self.sint32le = _io.read_s4le()?;
        self.sint64le = _io.read_s8le()?;
        self.magic_uint_be = _io.read_bytes(9 as usize)?.to_vec();
        self.uint16be = _io.read_u2be()?;
        self.uint32be = _io.read_u4be()?;
        self.uint64be = _io.read_u8be()?;
        self.magic_sint_be = _io.read_bytes(9 as usize)?.to_vec();
        self.sint16be = _io.read_s2be()?;
        self.sint32be = _io.read_s4be()?;
        self.sint64be = _io.read_s8be()?;
        Ok(())
    }
}
impl<'r, 's: 'r> Integers {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = Integers::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
