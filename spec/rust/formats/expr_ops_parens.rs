// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ExprOpsParens {
    pub bool_and: Option<i32>,
    pub str_0_to_4: Option<String>,
    pub bool_or: Option<i32>,
    pub f_e: Option<f64>,
    pub f_sum_to_int: Option<i32>,
    pub f_2pi: Option<f64>,
    pub str_concat_rev: Option<String>,
    pub i_m13: Option<i32>,
    pub str_concat_len: Option<i32>,
    pub str_concat_to_i: Option<i32>,
    pub i_42: Option<i8>,
    pub i_sum_to_str: Option<String>,
    pub bool_eq: Option<i32>,
    pub str_5_to_9: Option<String>,
    pub str_concat_substr_2_to_7: Option<String>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ExprOpsParens {
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
impl<'r, 's: 'r> ExprOpsParens {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprOpsParens::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn bool_and<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.bool_and.is_some() {
            return Ok(self.bool_and.as_ref().unwrap());
        }
        self.bool_and = Some( ((false) && (true))  as i32 as i32);
        return Ok(self.bool_and.as_ref().unwrap());
    }
    fn str_0_to_4<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.str_0_to_4.is_some() {
            return Ok(self.str_0_to_4.as_ref().unwrap());
        }
        self.str_0_to_4 = Some("01234".to_string());
        return Ok(self.str_0_to_4.as_ref().unwrap());
    }
    fn bool_or<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.bool_or.is_some() {
            return Ok(self.bool_or.as_ref().unwrap());
        }
        self.bool_or = Some( ((!(false)) || (false))  as i32 as i32);
        return Ok(self.bool_or.as_ref().unwrap());
    }
    fn f_e<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&f64> {
        if self.f_e.is_some() {
            return Ok(self.f_e.as_ref().unwrap());
        }
        self.f_e = Some(2.72 as f64);
        return Ok(self.f_e.as_ref().unwrap());
    }
    fn f_sum_to_int<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.f_sum_to_int.is_some() {
            return Ok(self.f_sum_to_int.as_ref().unwrap());
        }
        self.f_sum_to_int = Some((self.f_2pi(_io, _root, _parent)? + self.f_e(_io, _root, _parent)?) as i32 as i32);
        return Ok(self.f_sum_to_int.as_ref().unwrap());
    }
    fn f_2pi<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&f64> {
        if self.f_2pi.is_some() {
            return Ok(self.f_2pi.as_ref().unwrap());
        }
        self.f_2pi = Some(6.28 as f64);
        return Ok(self.f_2pi.as_ref().unwrap());
    }
    fn str_concat_rev<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.str_concat_rev.is_some() {
            return Ok(self.str_concat_rev.as_ref().unwrap());
        }
        self.str_concat_rev = Some(format!("{}{}", self.str_0_to_4(_io, _root, _parent)?, self.str_5_to_9(_io, _root, _parent)?).graphemes(true).rev().flat_map(|g| g.chars()).collect().to_string());
        return Ok(self.str_concat_rev.as_ref().unwrap());
    }
    fn i_m13<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.i_m13.is_some() {
            return Ok(self.i_m13.as_ref().unwrap());
        }
        self.i_m13 = Some(-13 as i32);
        return Ok(self.i_m13.as_ref().unwrap());
    }
    fn str_concat_len<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.str_concat_len.is_some() {
            return Ok(self.str_concat_len.as_ref().unwrap());
        }
        self.str_concat_len = Some(format!("{}{}", self.str_0_to_4(_io, _root, _parent)?, self.str_5_to_9(_io, _root, _parent)?).len() as i32);
        return Ok(self.str_concat_len.as_ref().unwrap());
    }
    fn str_concat_to_i<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.str_concat_to_i.is_some() {
            return Ok(self.str_concat_to_i.as_ref().unwrap());
        }
        self.str_concat_to_i = Some(format!("{}{}", self.str_0_to_4(_io, _root, _parent)?, self.str_5_to_9(_io, _root, _parent)?).parse().unwrap() as i32);
        return Ok(self.str_concat_to_i.as_ref().unwrap());
    }
    fn i_42<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i8> {
        if self.i_42.is_some() {
            return Ok(self.i_42.as_ref().unwrap());
        }
        self.i_42 = Some(42 as i8);
        return Ok(self.i_42.as_ref().unwrap());
    }
    fn i_sum_to_str<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.i_sum_to_str.is_some() {
            return Ok(self.i_sum_to_str.as_ref().unwrap());
        }
        self.i_sum_to_str = Some((self.i_42(_io, _root, _parent)? + self.i_m13(_io, _root, _parent)?).to_string().to_string());
        return Ok(self.i_sum_to_str.as_ref().unwrap());
    }
    fn bool_eq<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.bool_eq.is_some() {
            return Ok(self.bool_eq.as_ref().unwrap());
        }
        self.bool_eq = Some(false == true as i32 as i32);
        return Ok(self.bool_eq.as_ref().unwrap());
    }
    fn str_5_to_9<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.str_5_to_9.is_some() {
            return Ok(self.str_5_to_9.as_ref().unwrap());
        }
        self.str_5_to_9 = Some("56789".to_string());
        return Ok(self.str_5_to_9.as_ref().unwrap());
    }
    fn str_concat_substr_2_to_7<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.str_concat_substr_2_to_7.is_some() {
            return Ok(self.str_concat_substr_2_to_7.as_ref().unwrap());
        }
        self.str_concat_substr_2_to_7 = Some(format!("{}{}", self.str_0_to_4(_io, _root, _parent)?, self.str_5_to_9(_io, _root, _parent)?).substring(2, 7).to_string());
        return Ok(self.str_concat_substr_2_to_7.as_ref().unwrap());
    }
}
