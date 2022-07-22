// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ExprBytesCmp {
    pub one: Vec<u8>,
    pub two: Vec<u8>,
    pub is_le: Option<bool>,
    pub ack: Option<Vec<u8>>,
    pub is_gt2: Option<bool>,
    pub is_gt: Option<bool>,
    pub ack2: Option<Vec<u8>>,
    pub is_eq: Option<bool>,
    pub is_lt2: Option<bool>,
    pub is_ge: Option<bool>,
    pub hi_val: Option<Vec<u8>>,
    pub is_ne: Option<bool>,
    pub is_lt: Option<bool>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ExprBytesCmp {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.one = _io.read_bytes(1 as usize)?.to_vec();
        self.two = _io.read_bytes(3 as usize)?.to_vec();
        Ok(())
    }
}
impl<'r, 's: 'r> ExprBytesCmp {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprBytesCmp::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn is_le<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.is_le.is_some() {
            return Ok(self.is_le.as_ref().unwrap());
        }
        self.is_le = Some(self.two <= self.ack2(_io, _root, _parent)? as bool);
        return Ok(self.is_le.as_ref().unwrap());
    }
    fn ack<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&Vec<u8>> {
        if self.ack.is_some() {
            return Ok(self.ack.as_ref().unwrap());
        }
        self.ack = Some(&[0x41, 0x43, 0x4b] as Vec<u8>);
        return Ok(self.ack.as_ref().unwrap());
    }
    fn is_gt2<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.is_gt2.is_some() {
            return Ok(self.is_gt2.as_ref().unwrap());
        }
        self.is_gt2 = Some(self.hi_val(_io, _root, _parent)? > self.two as bool);
        return Ok(self.is_gt2.as_ref().unwrap());
    }
    fn is_gt<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.is_gt.is_some() {
            return Ok(self.is_gt.as_ref().unwrap());
        }
        self.is_gt = Some(self.two > self.ack2(_io, _root, _parent)? as bool);
        return Ok(self.is_gt.as_ref().unwrap());
    }
    fn ack2<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&Vec<u8>> {
        if self.ack2.is_some() {
            return Ok(self.ack2.as_ref().unwrap());
        }
        self.ack2 = Some(&[0x41, 0x43, 0x4b, 0x32] as Vec<u8>);
        return Ok(self.ack2.as_ref().unwrap());
    }
    fn is_eq<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.is_eq.is_some() {
            return Ok(self.is_eq.as_ref().unwrap());
        }
        self.is_eq = Some(self.two == self.ack(_io, _root, _parent)? as bool);
        return Ok(self.is_eq.as_ref().unwrap());
    }
    fn is_lt2<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.is_lt2.is_some() {
            return Ok(self.is_lt2.as_ref().unwrap());
        }
        self.is_lt2 = Some(self.one < self.two as bool);
        return Ok(self.is_lt2.as_ref().unwrap());
    }
    fn is_ge<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.is_ge.is_some() {
            return Ok(self.is_ge.as_ref().unwrap());
        }
        self.is_ge = Some(self.two >= self.ack2(_io, _root, _parent)? as bool);
        return Ok(self.is_ge.as_ref().unwrap());
    }
    fn hi_val<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&Vec<u8>> {
        if self.hi_val.is_some() {
            return Ok(self.hi_val.as_ref().unwrap());
        }
        self.hi_val = Some(&[0x90, 0x43] as Vec<u8>);
        return Ok(self.hi_val.as_ref().unwrap());
    }
    fn is_ne<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.is_ne.is_some() {
            return Ok(self.is_ne.as_ref().unwrap());
        }
        self.is_ne = Some(self.two != self.ack(_io, _root, _parent)? as bool);
        return Ok(self.is_ne.as_ref().unwrap());
    }
    fn is_lt<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.is_lt.is_some() {
            return Ok(self.is_lt.as_ref().unwrap());
        }
        self.is_lt = Some(self.two < self.ack2(_io, _root, _parent)? as bool);
        return Ok(self.is_lt.as_ref().unwrap());
    }
}
