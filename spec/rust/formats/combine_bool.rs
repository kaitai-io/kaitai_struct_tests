// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct CombineBool {
    pub bool_bit: bool,
    pub bool_calc: Option<bool>,
    pub bool_calc_bit: Option<bool>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for CombineBool {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.bool_bit = _io.read_bits_int(1)? != 0;
        Ok(())
    }
}
impl<'r, 's: 'r> CombineBool {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = CombineBool::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn bool_calc<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.bool_calc.is_some() {
            return Ok(self.bool_calc.as_ref().unwrap());
        }
        self.bool_calc = Some(false as bool);
        return Ok(self.bool_calc.as_ref().unwrap());
    }
    fn bool_calc_bit<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.bool_calc_bit.is_some() {
            return Ok(self.bool_calc_bit.as_ref().unwrap());
        }
        self.bool_calc_bit = Some(if true { self.bool_calc(_io, _root, _parent)? } else { self.bool_bit} as bool);
        return Ok(self.bool_calc_bit.as_ref().unwrap());
    }
}
