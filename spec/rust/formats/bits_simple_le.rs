// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct BitsSimpleLe {
    pub byte_1: u64,
    pub byte_2: u64,
    pub bits_a: bool,
    pub bits_b: u64,
    pub bits_c: u64,
    pub large_bits_1: u64,
    pub spacer: u64,
    pub large_bits_2: u64,
    pub normal_s2: i16,
    pub byte_8_9_10: u64,
    pub byte_11_to_14: u64,
    pub byte_15_to_19: u64,
    pub byte_20_to_27: u64,
    pub test_if_b1: Option<i8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for BitsSimpleLe {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.byte_1 = _io.read_bits_int(8)?;
        self.byte_2 = _io.read_bits_int(8)?;
        self.bits_a = _io.read_bits_int(1)? != 0;
        self.bits_b = _io.read_bits_int(3)?;
        self.bits_c = _io.read_bits_int(4)?;
        self.large_bits_1 = _io.read_bits_int(10)?;
        self.spacer = _io.read_bits_int(3)?;
        self.large_bits_2 = _io.read_bits_int(11)?;
        _io.align_to_byte()?;
        self.normal_s2 = _io.read_s2be()?;
        self.byte_8_9_10 = _io.read_bits_int(24)?;
        self.byte_11_to_14 = _io.read_bits_int(32)?;
        self.byte_15_to_19 = _io.read_bits_int(40)?;
        self.byte_20_to_27 = _io.read_bits_int(64)?;
        Ok(())
    }
}
impl<'r, 's: 'r> BitsSimpleLe {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = BitsSimpleLe::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn test_if_b1<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i8> {
        if self.test_if_b1.is_some() {
            return Ok(self.test_if_b1.as_ref().unwrap());
        }
        {
            // condIfHeader(Compare(Name(identifier(bits_a)),Eq,Bool(true)))
            self.test_if_b1 = Some(123 as i8);
        }
        return Ok(self.test_if_b1.as_ref().unwrap());
    }
}
