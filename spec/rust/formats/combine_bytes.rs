// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct CombineBytes {
    pub bytes_term: Vec<u8>,
    pub bytes_limit: Vec<u8>,
    pub bytes_eos: Vec<u8>,
    pub limit_or_calc: Option<Vec<u8>>,
    pub term_or_limit: Option<Vec<u8>>,
    pub limit_or_eos: Option<Vec<u8>>,
    pub eos_or_calc: Option<Vec<u8>>,
    pub term_or_calc: Option<Vec<u8>>,
    pub bytes_calc: Option<Vec<u8>>,
    pub term_or_eos: Option<Vec<u8>>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for CombineBytes {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.bytes_term = _io.read_bytes_term(124, false, true, true)?;
        self.bytes_limit = _io.read_bytes(4 as usize)?.to_vec();
        self.bytes_eos = _io.read_bytes_full()?;
        Ok(())
    }
}
impl<'r, 's: 'r> CombineBytes {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = CombineBytes::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn limit_or_calc<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&Vec<u8>> {
        if self.limit_or_calc.is_some() {
            return Ok(self.limit_or_calc.as_ref().unwrap());
        }
        self.limit_or_calc = Some(if false { self.bytes_limit } else { self.bytes_calc(_io, _root, _parent)?} as Vec<u8>);
        return Ok(self.limit_or_calc.as_ref().unwrap());
    }
    fn term_or_limit<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&Vec<u8>> {
        if self.term_or_limit.is_some() {
            return Ok(self.term_or_limit.as_ref().unwrap());
        }
        self.term_or_limit = Some(if true { self.bytes_term } else { self.bytes_limit} as Vec<u8>);
        return Ok(self.term_or_limit.as_ref().unwrap());
    }
    fn limit_or_eos<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&Vec<u8>> {
        if self.limit_or_eos.is_some() {
            return Ok(self.limit_or_eos.as_ref().unwrap());
        }
        self.limit_or_eos = Some(if true { self.bytes_limit } else { self.bytes_eos} as Vec<u8>);
        return Ok(self.limit_or_eos.as_ref().unwrap());
    }
    fn eos_or_calc<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&Vec<u8>> {
        if self.eos_or_calc.is_some() {
            return Ok(self.eos_or_calc.as_ref().unwrap());
        }
        self.eos_or_calc = Some(if true { self.bytes_eos } else { self.bytes_calc(_io, _root, _parent)?} as Vec<u8>);
        return Ok(self.eos_or_calc.as_ref().unwrap());
    }
    fn term_or_calc<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&Vec<u8>> {
        if self.term_or_calc.is_some() {
            return Ok(self.term_or_calc.as_ref().unwrap());
        }
        self.term_or_calc = Some(if true { self.bytes_term } else { self.bytes_calc(_io, _root, _parent)?} as Vec<u8>);
        return Ok(self.term_or_calc.as_ref().unwrap());
    }
    fn bytes_calc<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&Vec<u8>> {
        if self.bytes_calc.is_some() {
            return Ok(self.bytes_calc.as_ref().unwrap());
        }
        self.bytes_calc = Some(&[0x52, 0x6e, 0x44] as Vec<u8>);
        return Ok(self.bytes_calc.as_ref().unwrap());
    }
    fn term_or_eos<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&Vec<u8>> {
        if self.term_or_eos.is_some() {
            return Ok(self.term_or_eos.as_ref().unwrap());
        }
        self.term_or_eos = Some(if false { self.bytes_term } else { self.bytes_eos} as Vec<u8>);
        return Ok(self.term_or_eos.as_ref().unwrap());
    }
}
