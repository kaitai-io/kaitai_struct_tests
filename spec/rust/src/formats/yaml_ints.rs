// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct YamlInts {
    pub test_u4_dec: Option<i32>,
    pub test_u4_hex: Option<i32>,
    pub test_u8_dec: Option<i32>,
    pub test_u8_hex: Option<i32>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for YamlInts {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        Ok(())
    }
}
impl<'r, 's: 'r> YamlInts {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = YamlInts::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn test_u4_dec<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.test_u4_dec.is_some() {
            return Ok(self.test_u4_dec.as_ref().unwrap());
        }
        self.test_u4_dec = Some(4294967295 as i32);
        return Ok(self.test_u4_dec.as_ref().unwrap());
    }
    fn test_u4_hex<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.test_u4_hex.is_some() {
            return Ok(self.test_u4_hex.as_ref().unwrap());
        }
        self.test_u4_hex = Some(4294967295 as i32);
        return Ok(self.test_u4_hex.as_ref().unwrap());
    }
    fn test_u8_dec<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.test_u8_dec.is_some() {
            return Ok(self.test_u8_dec.as_ref().unwrap());
        }
        self.test_u8_dec = Some(18446744073709551615 as i32);
        return Ok(self.test_u8_dec.as_ref().unwrap());
    }
    fn test_u8_hex<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.test_u8_hex.is_some() {
            return Ok(self.test_u8_hex.as_ref().unwrap());
        }
        self.test_u8_hex = Some(18446744073709551615 as i32);
        return Ok(self.test_u8_hex.as_ref().unwrap());
    }
}
