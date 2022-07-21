// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct StrLiterals2 {
    pub dollar1: Option<String>,
    pub dollar2: Option<String>,
    pub hash: Option<String>,
    pub at_sign: Option<String>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for StrLiterals2 {
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
impl<'r, 's: 'r> StrLiterals2 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = StrLiterals2::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn dollar1<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.dollar1.is_some() {
            return Ok(self.dollar1.as_ref().unwrap());
        }
        self.dollar1 = Some("$foo".to_string());
        return Ok(self.dollar1.as_ref().unwrap());
    }
    fn dollar2<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.dollar2.is_some() {
            return Ok(self.dollar2.as_ref().unwrap());
        }
        self.dollar2 = Some("${foo}".to_string());
        return Ok(self.dollar2.as_ref().unwrap());
    }
    fn hash<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.hash.is_some() {
            return Ok(self.hash.as_ref().unwrap());
        }
        self.hash = Some("#{foo}".to_string());
        return Ok(self.hash.as_ref().unwrap());
    }
    fn at_sign<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.at_sign.is_some() {
            return Ok(self.at_sign.as_ref().unwrap());
        }
        self.at_sign = Some("@foo".to_string());
        return Ok(self.at_sign.as_ref().unwrap());
    }
}
