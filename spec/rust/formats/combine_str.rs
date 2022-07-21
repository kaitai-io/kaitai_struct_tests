// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct CombineStr {
    pub str_term: String,
    pub str_limit: String,
    pub str_eos: String,
    pub limit_or_calc_bytes: Option<String>,
    pub limit_or_calc: Option<String>,
    pub term_or_limit: Option<String>,
    pub limit_or_eos: Option<String>,
    pub calc_or_calc_bytes: Option<String>,
    pub str_calc_bytes: Option<String>,
    pub eos_or_calc: Option<String>,
    pub term_or_calc: Option<String>,
    pub term_or_calc_bytes: Option<String>,
    pub term_or_eos: Option<String>,
    pub str_calc: Option<String>,
    pub eos_or_calc_bytes: Option<String>,
    pub calc_bytes: Option<Vec<u8>>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for CombineStr {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.str_term = decode_string(_io.read_bytes_term(124, false, true, true)?, "ASCII")?;
        self.str_limit = decode_string(_io.read_bytes(4 as usize)?, "ASCII")?;
        self.str_eos = decode_string(_io.read_bytes_full()?, "ASCII")?;
        Ok(())
    }
}
impl<'r, 's: 'r> CombineStr {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = CombineStr::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn limit_or_calc_bytes<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.limit_or_calc_bytes.is_some() {
            return Ok(self.limit_or_calc_bytes.as_ref().unwrap());
        }
        self.limit_or_calc_bytes = Some(if true { self.str_limit } else { self.str_calc_bytes(_io, _root, _parent)?}.to_string());
        return Ok(self.limit_or_calc_bytes.as_ref().unwrap());
    }
    fn limit_or_calc<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.limit_or_calc.is_some() {
            return Ok(self.limit_or_calc.as_ref().unwrap());
        }
        self.limit_or_calc = Some(if false { self.str_limit } else { self.str_calc(_io, _root, _parent)?}.to_string());
        return Ok(self.limit_or_calc.as_ref().unwrap());
    }
    fn term_or_limit<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.term_or_limit.is_some() {
            return Ok(self.term_or_limit.as_ref().unwrap());
        }
        self.term_or_limit = Some(if true { self.str_term } else { self.str_limit}.to_string());
        return Ok(self.term_or_limit.as_ref().unwrap());
    }
    fn limit_or_eos<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.limit_or_eos.is_some() {
            return Ok(self.limit_or_eos.as_ref().unwrap());
        }
        self.limit_or_eos = Some(if true { self.str_limit } else { self.str_eos}.to_string());
        return Ok(self.limit_or_eos.as_ref().unwrap());
    }
    fn calc_or_calc_bytes<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.calc_or_calc_bytes.is_some() {
            return Ok(self.calc_or_calc_bytes.as_ref().unwrap());
        }
        self.calc_or_calc_bytes = Some(if false { self.str_calc(_io, _root, _parent)? } else { self.str_calc_bytes(_io, _root, _parent)?}.to_string());
        return Ok(self.calc_or_calc_bytes.as_ref().unwrap());
    }
    fn str_calc_bytes<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.str_calc_bytes.is_some() {
            return Ok(self.str_calc_bytes.as_ref().unwrap());
        }
        self.str_calc_bytes = Some(decode_string(self.calc_bytes(_io, _root, _parent)?, "ASCII")?.to_string());
        return Ok(self.str_calc_bytes.as_ref().unwrap());
    }
    fn eos_or_calc<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.eos_or_calc.is_some() {
            return Ok(self.eos_or_calc.as_ref().unwrap());
        }
        self.eos_or_calc = Some(if false { self.str_eos } else { self.str_calc(_io, _root, _parent)?}.to_string());
        return Ok(self.eos_or_calc.as_ref().unwrap());
    }
    fn term_or_calc<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.term_or_calc.is_some() {
            return Ok(self.term_or_calc.as_ref().unwrap());
        }
        self.term_or_calc = Some(if true { self.str_term } else { self.str_calc(_io, _root, _parent)?}.to_string());
        return Ok(self.term_or_calc.as_ref().unwrap());
    }
    fn term_or_calc_bytes<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.term_or_calc_bytes.is_some() {
            return Ok(self.term_or_calc_bytes.as_ref().unwrap());
        }
        self.term_or_calc_bytes = Some(if false { self.str_term } else { self.str_calc_bytes(_io, _root, _parent)?}.to_string());
        return Ok(self.term_or_calc_bytes.as_ref().unwrap());
    }
    fn term_or_eos<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.term_or_eos.is_some() {
            return Ok(self.term_or_eos.as_ref().unwrap());
        }
        self.term_or_eos = Some(if false { self.str_term } else { self.str_eos}.to_string());
        return Ok(self.term_or_eos.as_ref().unwrap());
    }
    fn str_calc<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.str_calc.is_some() {
            return Ok(self.str_calc.as_ref().unwrap());
        }
        self.str_calc = Some("bar".to_string());
        return Ok(self.str_calc.as_ref().unwrap());
    }
    fn eos_or_calc_bytes<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.eos_or_calc_bytes.is_some() {
            return Ok(self.eos_or_calc_bytes.as_ref().unwrap());
        }
        self.eos_or_calc_bytes = Some(if true { self.str_eos } else { self.str_calc_bytes(_io, _root, _parent)?}.to_string());
        return Ok(self.eos_or_calc_bytes.as_ref().unwrap());
    }
    fn calc_bytes<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&Vec<u8>> {
        if self.calc_bytes.is_some() {
            return Ok(self.calc_bytes.as_ref().unwrap());
        }
        self.calc_bytes = Some(&[0x62, 0x61, 0x7a] as Vec<u8>);
        return Ok(self.calc_bytes.as_ref().unwrap());
    }
}
