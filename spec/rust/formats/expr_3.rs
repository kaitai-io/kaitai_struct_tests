// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct Expr3 {
    pub one: u8,
    pub two: String,
    pub three: Option<String>,
    pub is_str_ge: Option<bool>,
    pub is_str_ne: Option<bool>,
    pub is_str_gt: Option<bool>,
    pub is_str_le: Option<bool>,
    pub is_str_lt2: Option<bool>,
    pub test_not: Option<bool>,
    pub is_str_lt: Option<bool>,
    pub four: Option<String>,
    pub is_str_eq: Option<bool>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for Expr3 {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.one = _io.read_u1()?;
        self.two = decode_string(_io.read_bytes(3 as usize)?, "ASCII")?;
        Ok(())
    }
}
impl<'r, 's: 'r> Expr3 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = Expr3::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn three<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.three.is_some() {
            return Ok(self.three.as_ref().unwrap());
        }
        self.three = Some(format!("{}{}", "@", self.two).to_string());
        return Ok(self.three.as_ref().unwrap());
    }
    fn is_str_ge<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.is_str_ge.is_some() {
            return Ok(self.is_str_ge.as_ref().unwrap());
        }
        self.is_str_ge = Some(self.two >= "ACK2" as bool);
        return Ok(self.is_str_ge.as_ref().unwrap());
    }
    fn is_str_ne<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.is_str_ne.is_some() {
            return Ok(self.is_str_ne.as_ref().unwrap());
        }
        self.is_str_ne = Some(self.two != "ACK" as bool);
        return Ok(self.is_str_ne.as_ref().unwrap());
    }
    fn is_str_gt<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.is_str_gt.is_some() {
            return Ok(self.is_str_gt.as_ref().unwrap());
        }
        self.is_str_gt = Some(self.two > "ACK2" as bool);
        return Ok(self.is_str_gt.as_ref().unwrap());
    }
    fn is_str_le<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.is_str_le.is_some() {
            return Ok(self.is_str_le.as_ref().unwrap());
        }
        self.is_str_le = Some(self.two <= "ACK2" as bool);
        return Ok(self.is_str_le.as_ref().unwrap());
    }
    fn is_str_lt2<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.is_str_lt2.is_some() {
            return Ok(self.is_str_lt2.as_ref().unwrap());
        }
        self.is_str_lt2 = Some(self.three(_io, _root, _parent)? < self.two as bool);
        return Ok(self.is_str_lt2.as_ref().unwrap());
    }
    fn test_not<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.test_not.is_some() {
            return Ok(self.test_not.as_ref().unwrap());
        }
        self.test_not = Some(!(false) as bool);
        return Ok(self.test_not.as_ref().unwrap());
    }
    fn is_str_lt<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.is_str_lt.is_some() {
            return Ok(self.is_str_lt.as_ref().unwrap());
        }
        self.is_str_lt = Some(self.two < "ACK2" as bool);
        return Ok(self.is_str_lt.as_ref().unwrap());
    }
    fn four<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.four.is_some() {
            return Ok(self.four.as_ref().unwrap());
        }
        self.four = Some(format!("{}{}", format!("{}{}", "_", self.two), "_").to_string());
        return Ok(self.four.as_ref().unwrap());
    }
    fn is_str_eq<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.is_str_eq.is_some() {
            return Ok(self.is_str_eq.as_ref().unwrap());
        }
        self.is_str_eq = Some(self.two == "ACK" as bool);
        return Ok(self.is_str_eq.as_ref().unwrap());
    }
}
