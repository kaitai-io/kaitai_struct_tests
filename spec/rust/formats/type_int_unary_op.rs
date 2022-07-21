// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct TypeIntUnaryOp {
    pub value_s2: i16,
    pub value_s8: i64,
    pub unary_s2: Option<i32>,
    pub unary_s8: Option<i64>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for TypeIntUnaryOp {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.value_s2 = _io.read_s2le()?;
        self.value_s8 = _io.read_s8le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> TypeIntUnaryOp {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = TypeIntUnaryOp::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn unary_s2<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.unary_s2.is_some() {
            return Ok(self.unary_s2.as_ref().unwrap());
        }
        self.unary_s2 = Some(-(self.value_s2) as i32);
        return Ok(self.unary_s2.as_ref().unwrap());
    }
    fn unary_s8<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i64> {
        if self.unary_s8.is_some() {
            return Ok(self.unary_s8.as_ref().unwrap());
        }
        self.unary_s8 = Some(-(self.value_s8) as i64);
        return Ok(self.unary_s8.as_ref().unwrap());
    }
}
