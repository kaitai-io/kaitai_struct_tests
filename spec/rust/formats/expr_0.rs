// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct Expr0 {
    pub len_of_1: u16,
    pub must_be_f7: Option<i32>,
    pub must_be_abc123: Option<String>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for Expr0 {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.len_of_1 = _io.read_u2le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> Expr0 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = Expr0::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn must_be_f7<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.must_be_f7.is_some() {
            return Ok(self.must_be_f7.as_ref().unwrap());
        }
        self.must_be_f7 = Some((7 + 240) as i32);
        return Ok(self.must_be_f7.as_ref().unwrap());
    }
    fn must_be_abc123<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.must_be_abc123.is_some() {
            return Ok(self.must_be_abc123.as_ref().unwrap());
        }
        self.must_be_abc123 = Some(format!("{}{}", "abc", "123").to_string());
        return Ok(self.must_be_abc123.as_ref().unwrap());
    }
}
