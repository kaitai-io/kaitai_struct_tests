// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ExprStrEncodings {
    pub len_of_1: u16,
    pub str1: String,
    pub len_of_2: u16,
    pub str2: String,
    pub len_of_3: u16,
    pub str3: String,
    pub len_of_4: u16,
    pub str4: String,
    pub str4_gt_str_from_bytes: Option<bool>,
    pub str1_eq: Option<bool>,
    pub str4_eq: Option<bool>,
    pub str3_eq_str2: Option<bool>,
    pub str4_gt_str_calc: Option<bool>,
    pub str2_eq: Option<bool>,
    pub str3_eq: Option<bool>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ExprStrEncodings {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.len_of_1 = _io.read_u2le()?;
        self.str1 = decode_string(_io.read_bytes(self.len_of_1 as usize)?, "ASCII")?;
        self.len_of_2 = _io.read_u2le()?;
        self.str2 = decode_string(_io.read_bytes(self.len_of_2 as usize)?, "UTF-8")?;
        self.len_of_3 = _io.read_u2le()?;
        self.str3 = decode_string(_io.read_bytes(self.len_of_3 as usize)?, "SJIS")?;
        self.len_of_4 = _io.read_u2le()?;
        self.str4 = decode_string(_io.read_bytes(self.len_of_4 as usize)?, "CP437")?;
        Ok(())
    }
}
impl<'r, 's: 'r> ExprStrEncodings {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprStrEncodings::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn str4_gt_str_from_bytes<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.str4_gt_str_from_bytes.is_some() {
            return Ok(self.str4_gt_str_from_bytes.as_ref().unwrap());
        }
        self.str4_gt_str_from_bytes = Some(self.str4 > decode_string(&[0xb4], "CP437")? as bool);
        return Ok(self.str4_gt_str_from_bytes.as_ref().unwrap());
    }
    fn str1_eq<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.str1_eq.is_some() {
            return Ok(self.str1_eq.as_ref().unwrap());
        }
        self.str1_eq = Some(self.str1 == "Some ASCII" as bool);
        return Ok(self.str1_eq.as_ref().unwrap());
    }
    fn str4_eq<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.str4_eq.is_some() {
            return Ok(self.str4_eq.as_ref().unwrap());
        }
        self.str4_eq = Some(self.str4 == "\u{2591}\u{2592}\u{2593}" as bool);
        return Ok(self.str4_eq.as_ref().unwrap());
    }
    fn str3_eq_str2<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.str3_eq_str2.is_some() {
            return Ok(self.str3_eq_str2.as_ref().unwrap());
        }
        self.str3_eq_str2 = Some(self.str3 == self.str2 as bool);
        return Ok(self.str3_eq_str2.as_ref().unwrap());
    }
    fn str4_gt_str_calc<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.str4_gt_str_calc.is_some() {
            return Ok(self.str4_gt_str_calc.as_ref().unwrap());
        }
        self.str4_gt_str_calc = Some(self.str4 > "\u{2524}" as bool);
        return Ok(self.str4_gt_str_calc.as_ref().unwrap());
    }
    fn str2_eq<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.str2_eq.is_some() {
            return Ok(self.str2_eq.as_ref().unwrap());
        }
        self.str2_eq = Some(self.str2 == "\u{3053}\u{3093}\u{306b}\u{3061}\u{306f}" as bool);
        return Ok(self.str2_eq.as_ref().unwrap());
    }
    fn str3_eq<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.str3_eq.is_some() {
            return Ok(self.str3_eq.as_ref().unwrap());
        }
        self.str3_eq = Some(self.str3 == "\u{3053}\u{3093}\u{306b}\u{3061}\u{306f}" as bool);
        return Ok(self.str3_eq.as_ref().unwrap());
    }
}
