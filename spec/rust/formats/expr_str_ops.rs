// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ExprStrOps {
    pub one: String,
    pub one_substr_3_to_3: Option<String>,
    pub to_i_r8: Option<i32>,
    pub to_i_r16: Option<i32>,
    pub two_substr_0_to_10: Option<String>,
    pub one_len: Option<i32>,
    pub two_len: Option<i32>,
    pub one_substr_2_to_5: Option<String>,
    pub to_i_r2: Option<i32>,
    pub two_rev: Option<String>,
    pub two: Option<String>,
    pub two_substr_4_to_10: Option<String>,
    pub to_i_r10: Option<i32>,
    pub two_substr_0_to_7: Option<String>,
    pub to_i_attr: Option<i32>,
    pub one_substr_0_to_3: Option<String>,
    pub one_rev: Option<String>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ExprStrOps {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.one = decode_string(_io.read_bytes(5 as usize)?, "ASCII")?;
        Ok(())
    }
}
impl<'r, 's: 'r> ExprStrOps {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprStrOps::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn one_substr_3_to_3<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.one_substr_3_to_3.is_some() {
            return Ok(self.one_substr_3_to_3.as_ref().unwrap());
        }
        self.one_substr_3_to_3 = Some(self.one.substring(3, 3).to_string());
        return Ok(self.one_substr_3_to_3.as_ref().unwrap());
    }
    fn to_i_r8<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.to_i_r8.is_some() {
            return Ok(self.to_i_r8.as_ref().unwrap());
        }
        self.to_i_r8 = Some(panic!("Converting from string to int in base {} is unimplemented"8) as i32);
        return Ok(self.to_i_r8.as_ref().unwrap());
    }
    fn to_i_r16<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.to_i_r16.is_some() {
            return Ok(self.to_i_r16.as_ref().unwrap());
        }
        self.to_i_r16 = Some(panic!("Converting from string to int in base {} is unimplemented"16) as i32);
        return Ok(self.to_i_r16.as_ref().unwrap());
    }
    fn two_substr_0_to_10<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.two_substr_0_to_10.is_some() {
            return Ok(self.two_substr_0_to_10.as_ref().unwrap());
        }
        self.two_substr_0_to_10 = Some(self.two(_io, _root, _parent)?.substring(0, 10).to_string());
        return Ok(self.two_substr_0_to_10.as_ref().unwrap());
    }
    fn one_len<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.one_len.is_some() {
            return Ok(self.one_len.as_ref().unwrap());
        }
        self.one_len = Some(self.one.len() as i32);
        return Ok(self.one_len.as_ref().unwrap());
    }
    fn two_len<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.two_len.is_some() {
            return Ok(self.two_len.as_ref().unwrap());
        }
        self.two_len = Some(self.two(_io, _root, _parent)?.len() as i32);
        return Ok(self.two_len.as_ref().unwrap());
    }
    fn one_substr_2_to_5<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.one_substr_2_to_5.is_some() {
            return Ok(self.one_substr_2_to_5.as_ref().unwrap());
        }
        self.one_substr_2_to_5 = Some(self.one.substring(2, 5).to_string());
        return Ok(self.one_substr_2_to_5.as_ref().unwrap());
    }
    fn to_i_r2<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.to_i_r2.is_some() {
            return Ok(self.to_i_r2.as_ref().unwrap());
        }
        self.to_i_r2 = Some(panic!("Converting from string to int in base {} is unimplemented"2) as i32);
        return Ok(self.to_i_r2.as_ref().unwrap());
    }
    fn two_rev<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.two_rev.is_some() {
            return Ok(self.two_rev.as_ref().unwrap());
        }
        self.two_rev = Some(self.two(_io, _root, _parent)?.graphemes(true).rev().flat_map(|g| g.chars()).collect().to_string());
        return Ok(self.two_rev.as_ref().unwrap());
    }
    fn two<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.two.is_some() {
            return Ok(self.two.as_ref().unwrap());
        }
        self.two = Some("0123456789".to_string());
        return Ok(self.two.as_ref().unwrap());
    }
    fn two_substr_4_to_10<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.two_substr_4_to_10.is_some() {
            return Ok(self.two_substr_4_to_10.as_ref().unwrap());
        }
        self.two_substr_4_to_10 = Some(self.two(_io, _root, _parent)?.substring(4, 10).to_string());
        return Ok(self.two_substr_4_to_10.as_ref().unwrap());
    }
    fn to_i_r10<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.to_i_r10.is_some() {
            return Ok(self.to_i_r10.as_ref().unwrap());
        }
        self.to_i_r10 = Some("-072".parse().unwrap() as i32);
        return Ok(self.to_i_r10.as_ref().unwrap());
    }
    fn two_substr_0_to_7<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.two_substr_0_to_7.is_some() {
            return Ok(self.two_substr_0_to_7.as_ref().unwrap());
        }
        self.two_substr_0_to_7 = Some(self.two(_io, _root, _parent)?.substring(0, 7).to_string());
        return Ok(self.two_substr_0_to_7.as_ref().unwrap());
    }
    fn to_i_attr<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.to_i_attr.is_some() {
            return Ok(self.to_i_attr.as_ref().unwrap());
        }
        self.to_i_attr = Some("9173".parse().unwrap() as i32);
        return Ok(self.to_i_attr.as_ref().unwrap());
    }
    fn one_substr_0_to_3<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.one_substr_0_to_3.is_some() {
            return Ok(self.one_substr_0_to_3.as_ref().unwrap());
        }
        self.one_substr_0_to_3 = Some(self.one.substring(0, 3).to_string());
        return Ok(self.one_substr_0_to_3.as_ref().unwrap());
    }
    fn one_rev<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.one_rev.is_some() {
            return Ok(self.one_rev.as_ref().unwrap());
        }
        self.one_rev = Some(self.one.graphemes(true).rev().flat_map(|g| g.chars()).collect().to_string());
        return Ok(self.one_rev.as_ref().unwrap());
    }
}
