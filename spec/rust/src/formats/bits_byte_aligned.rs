// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct BitsByteAligned {
    pub one: u64,
    pub byte_1: u8,
    pub two: u64,
    pub three: bool,
    pub byte_2: u8,
    pub four: u64,
    pub byte_3: Vec<u8>,
    pub full_byte: u64,
    pub byte_4: u8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for BitsByteAligned {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.one = _io.read_bits_int(6)?;
        _io.align_to_byte()?;
        self.byte_1 = _io.read_u1()?;
        self.two = _io.read_bits_int(3)?;
        self.three = _io.read_bits_int(1)? != 0;
        _io.align_to_byte()?;
        self.byte_2 = _io.read_u1()?;
        self.four = _io.read_bits_int(14)?;
        _io.align_to_byte()?;
        self.byte_3 = _io.read_bytes(1 as usize)?.to_vec();
        self.full_byte = _io.read_bits_int(8)?;
        _io.align_to_byte()?;
        self.byte_4 = _io.read_u1()?;
        Ok(())
    }
}
impl<'r, 's: 'r> BitsByteAligned {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = BitsByteAligned::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
