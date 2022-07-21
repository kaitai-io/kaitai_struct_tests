// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ExprBytesOps {
    pub one: Vec<u8>,
    pub two_last: Option<u8>,
    pub two_max: Option<u8>,
    pub one_min: Option<u8>,
    pub one_first: Option<u8>,
    pub one_mid: Option<u8>,
    pub two: Option<Vec<u8>>,
    pub two_min: Option<u8>,
    pub two_mid: Option<u8>,
    pub one_size: Option<i32>,
    pub one_last: Option<u8>,
    pub two_size: Option<i32>,
    pub one_max: Option<u8>,
    pub two_first: Option<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ExprBytesOps {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.one = _io.read_bytes(3 as usize)?.to_vec();
        Ok(())
    }
}
impl<'r, 's: 'r> ExprBytesOps {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprBytesOps::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn two_last<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u8> {
        if self.two_last.is_some() {
            return Ok(self.two_last.as_ref().unwrap());
        }
        self.two_last = Some(self.two(_io, _root, _parent)?.last() as u8);
        return Ok(self.two_last.as_ref().unwrap());
    }
    fn two_max<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u8> {
        if self.two_max.is_some() {
            return Ok(self.two_max.as_ref().unwrap());
        }
        self.two_max = Some(self.two(_io, _root, _parent)?.iter().max() as u8);
        return Ok(self.two_max.as_ref().unwrap());
    }
    fn one_min<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u8> {
        if self.one_min.is_some() {
            return Ok(self.one_min.as_ref().unwrap());
        }
        self.one_min = Some(self.one.iter().min() as u8);
        return Ok(self.one_min.as_ref().unwrap());
    }
    fn one_first<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u8> {
        if self.one_first.is_some() {
            return Ok(self.one_first.as_ref().unwrap());
        }
        self.one_first = Some(self.one.first() as u8);
        return Ok(self.one_first.as_ref().unwrap());
    }
    fn one_mid<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u8> {
        if self.one_mid.is_some() {
            return Ok(self.one_mid.as_ref().unwrap());
        }
        self.one_mid = Some(self.one[1 as usize] as u8);
        return Ok(self.one_mid.as_ref().unwrap());
    }
    fn two<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&Vec<u8>> {
        if self.two.is_some() {
            return Ok(self.two.as_ref().unwrap());
        }
        self.two = Some(&[0x41, 0xff, 0x4b] as Vec<u8>);
        return Ok(self.two.as_ref().unwrap());
    }
    fn two_min<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u8> {
        if self.two_min.is_some() {
            return Ok(self.two_min.as_ref().unwrap());
        }
        self.two_min = Some(self.two(_io, _root, _parent)?.iter().min() as u8);
        return Ok(self.two_min.as_ref().unwrap());
    }
    fn two_mid<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u8> {
        if self.two_mid.is_some() {
            return Ok(self.two_mid.as_ref().unwrap());
        }
        self.two_mid = Some(self.two(_io, _root, _parent)?[1 as usize] as u8);
        return Ok(self.two_mid.as_ref().unwrap());
    }
    fn one_size<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.one_size.is_some() {
            return Ok(self.one_size.as_ref().unwrap());
        }
        self.one_size = Some(self.one.len() as i32);
        return Ok(self.one_size.as_ref().unwrap());
    }
    fn one_last<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u8> {
        if self.one_last.is_some() {
            return Ok(self.one_last.as_ref().unwrap());
        }
        self.one_last = Some(self.one.last() as u8);
        return Ok(self.one_last.as_ref().unwrap());
    }
    fn two_size<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.two_size.is_some() {
            return Ok(self.two_size.as_ref().unwrap());
        }
        self.two_size = Some(self.two(_io, _root, _parent)?.len() as i32);
        return Ok(self.two_size.as_ref().unwrap());
    }
    fn one_max<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u8> {
        if self.one_max.is_some() {
            return Ok(self.one_max.as_ref().unwrap());
        }
        self.one_max = Some(self.one.iter().max() as u8);
        return Ok(self.one_max.as_ref().unwrap());
    }
    fn two_first<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u8> {
        if self.two_first.is_some() {
            return Ok(self.two_first.as_ref().unwrap());
        }
        self.two_first = Some(self.two(_io, _root, _parent)?.first() as u8);
        return Ok(self.two_first.as_ref().unwrap());
    }
}
