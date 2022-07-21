// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct IntegersDoubleOverflow {
    pub signed_safe_min_be: i64,
    pub signed_safe_min_le: i64,
    pub signed_safe_max_be: i64,
    pub signed_safe_max_le: i64,
    pub signed_unsafe_neg_be: i64,
    pub signed_unsafe_neg_le: i64,
    pub signed_unsafe_pos_be: i64,
    pub signed_unsafe_pos_le: i64,
    pub unsigned_safe_max_be: Option<u64>,
    pub unsigned_safe_max_le: Option<u64>,
    pub unsigned_unsafe_pos_be: Option<u64>,
    pub unsigned_unsafe_pos_le: Option<u64>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for IntegersDoubleOverflow {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.signed_safe_min_be = _io.read_s8be()?;
        self.signed_safe_min_le = _io.read_s8le()?;
        self.signed_safe_max_be = _io.read_s8be()?;
        self.signed_safe_max_le = _io.read_s8le()?;
        self.signed_unsafe_neg_be = _io.read_s8be()?;
        self.signed_unsafe_neg_le = _io.read_s8le()?;
        self.signed_unsafe_pos_be = _io.read_s8be()?;
        self.signed_unsafe_pos_le = _io.read_s8le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> IntegersDoubleOverflow {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = IntegersDoubleOverflow::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn unsigned_safe_max_be<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u64> {
        if self.unsigned_safe_max_be.is_some() {
            return Ok(self.unsigned_safe_max_be.as_ref().unwrap());
        }
        // pushPos(_io)
        // seek(_io, IntNum(16))
        // popPos(_io)
        return Ok(self.unsigned_safe_max_be.as_ref().unwrap());
    }
    fn unsigned_safe_max_le<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u64> {
        if self.unsigned_safe_max_le.is_some() {
            return Ok(self.unsigned_safe_max_le.as_ref().unwrap());
        }
        // pushPos(_io)
        // seek(_io, IntNum(24))
        // popPos(_io)
        return Ok(self.unsigned_safe_max_le.as_ref().unwrap());
    }
    fn unsigned_unsafe_pos_be<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u64> {
        if self.unsigned_unsafe_pos_be.is_some() {
            return Ok(self.unsigned_unsafe_pos_be.as_ref().unwrap());
        }
        // pushPos(_io)
        // seek(_io, IntNum(48))
        // popPos(_io)
        return Ok(self.unsigned_unsafe_pos_be.as_ref().unwrap());
    }
    fn unsigned_unsafe_pos_le<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u64> {
        if self.unsigned_unsafe_pos_le.is_some() {
            return Ok(self.unsigned_unsafe_pos_le.as_ref().unwrap());
        }
        // pushPos(_io)
        // seek(_io, IntNum(56))
        // popPos(_io)
        return Ok(self.unsigned_unsafe_pos_le.as_ref().unwrap());
    }
}
