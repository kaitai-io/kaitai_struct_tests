// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct Expr1 {
    pub len_of_1: u16,
    pub str1: String,
    pub len_of_1_mod: Option<i32>,
    pub str1_len: Option<i32>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for Expr1 {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.len_of_1 = _io.read_u2le()?;
        self.str1 = decode_string(_io.read_bytes(self.len_of_1_mod(_io, _root, _parent)? as usize)?, "ASCII")?;
        Ok(())
    }
}
impl<'r, 's: 'r> Expr1 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = Expr1::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn len_of_1_mod<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.len_of_1_mod.is_some() {
            return Ok(self.len_of_1_mod.as_ref().unwrap());
        }
        self.len_of_1_mod = Some((self.len_of_1 - 2) as i32);
        return Ok(self.len_of_1_mod.as_ref().unwrap());
    }
    fn str1_len<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.str1_len.is_some() {
            return Ok(self.str1_len.as_ref().unwrap());
        }
        self.str1_len = Some(self.str1.len() as i32);
        return Ok(self.str1_len.as_ref().unwrap());
    }
}
