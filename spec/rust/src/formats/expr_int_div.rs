// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ExprIntDiv {
    pub int_u: u32,
    pub int_s: i32,
    pub div_pos_const: Option<i32>,
    pub div_neg_const: Option<i32>,
    pub div_pos_seq: Option<i32>,
    pub div_neg_seq: Option<i32>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ExprIntDiv {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.int_u = _io.read_u4le()?;
        self.int_s = _io.read_s4le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> ExprIntDiv {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprIntDiv::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn div_pos_const<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.div_pos_const.is_some() {
            return Ok(self.div_pos_const.as_ref().unwrap());
        }
        self.div_pos_const = Some(9837 / 13 as i32);
        return Ok(self.div_pos_const.as_ref().unwrap());
    }
    fn div_neg_const<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.div_neg_const.is_some() {
            return Ok(self.div_neg_const.as_ref().unwrap());
        }
        self.div_neg_const = Some(-9837 / 13 as i32);
        return Ok(self.div_neg_const.as_ref().unwrap());
    }
    fn div_pos_seq<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.div_pos_seq.is_some() {
            return Ok(self.div_pos_seq.as_ref().unwrap());
        }
        self.div_pos_seq = Some(self.int_u / 13 as i32);
        return Ok(self.div_pos_seq.as_ref().unwrap());
    }
    fn div_neg_seq<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.div_neg_seq.is_some() {
            return Ok(self.div_neg_seq.as_ref().unwrap());
        }
        self.div_neg_seq = Some(self.int_s / 13 as i32);
        return Ok(self.div_neg_seq.as_ref().unwrap());
    }
}
