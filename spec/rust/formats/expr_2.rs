// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct Expr2 {
    pub str1: Option<Expr2_ModStr>,
    pub str2: Option<Expr2_ModStr>,
    pub str1_len_mod: Option<i32>,
    pub str1_len: Option<i32>,
    pub str1_tuple5: Option<Expr2_Tuple>,
    pub str2_tuple5: Option<Expr2_Tuple>,
    pub str1_avg: Option<i32>,
    pub str1_byte1: Option<u8>,
    pub str1_char5: Option<String>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for Expr2 {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.str1 = Some(Self::read_into::<BytesReader, Expr2_ModStr>(Self::read_into::<S, Expr2_ModStr>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.str2 = Some(Self::read_into::<BytesReader, Expr2_ModStr>(Self::read_into::<S, Expr2_ModStr>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> Expr2 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = Expr2::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn str1_len_mod<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.str1_len_mod.is_some() {
            return Ok(self.str1_len_mod.as_ref().unwrap());
        }
        self.str1_len_mod = Some(self.str1.len_mod as i32);
        return Ok(self.str1_len_mod.as_ref().unwrap());
    }
    fn str1_len<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.str1_len.is_some() {
            return Ok(self.str1_len.as_ref().unwrap());
        }
        self.str1_len = Some(self.str1.str.len() as i32);
        return Ok(self.str1_len.as_ref().unwrap());
    }
    fn str1_tuple5<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&Expr2_Tuple> {
        if self.str1_tuple5.is_some() {
            return Ok(self.str1_tuple5.as_ref().unwrap());
        }
        self.str1_tuple5 = Some(self.str1.tuple5 as Tuple);
        return Ok(self.str1_tuple5.as_ref().unwrap());
    }
    fn str2_tuple5<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&Expr2_Tuple> {
        if self.str2_tuple5.is_some() {
            return Ok(self.str2_tuple5.as_ref().unwrap());
        }
        self.str2_tuple5 = Some(self.str2.tuple5 as Tuple);
        return Ok(self.str2_tuple5.as_ref().unwrap());
    }
    fn str1_avg<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.str1_avg.is_some() {
            return Ok(self.str1_avg.as_ref().unwrap());
        }
        self.str1_avg = Some(self.str1.rest.avg as i32);
        return Ok(self.str1_avg.as_ref().unwrap());
    }
    fn str1_byte1<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u8> {
        if self.str1_byte1.is_some() {
            return Ok(self.str1_byte1.as_ref().unwrap());
        }
        self.str1_byte1 = Some(self.str1.rest.byte1 as u8);
        return Ok(self.str1_byte1.as_ref().unwrap());
    }
    fn str1_char5<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.str1_char5.is_some() {
            return Ok(self.str1_char5.as_ref().unwrap());
        }
        self.str1_char5 = Some(self.str1.char5.to_string());
        return Ok(self.str1_char5.as_ref().unwrap());
    }
}
// extraAttrForIO(RawIdentifier(NamedIdentifier(rest)), NoRepeat)

#[derive(Default, Debug, PartialEq)]
pub struct Expr2_ModStr {
    pub len_orig: u16,
    pub str: String,
    pub rest: Option<Expr2_Tuple>,
    pub raw_rest: Vec<u8>,
    pub len_mod: Option<i32>,
    pub char5: Option<String>,
    pub tuple5: Option<Expr2_Tuple>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for Expr2_ModStr {
    type Root = Expr2;
    type ParentStack = (&'r Expr2, <Expr2 as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.len_orig = _io.read_u2le()?;
        self.str = decode_string(_io.read_bytes(self.len_mod(_io, _root, _parent)? as usize)?, "UTF-8")?;
        self.rest = Some(Self::read_into::<BytesReader, Expr2_Tuple>(&BytesReader::new(_io.read_bytes(3 as usize)?), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> Expr2_ModStr {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = Expr2_ModStr::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn len_mod<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Expr2>,
        _parent: Option<TypedStack<(&'r Expr2, <Expr2 as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.len_mod.is_some() {
            return Ok(self.len_mod.as_ref().unwrap());
        }
        self.len_mod = Some((self.len_orig - 3) as i32);
        return Ok(self.len_mod.as_ref().unwrap());
    }
    fn char5<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Expr2>,
        _parent: Option<TypedStack<(&'r Expr2, <Expr2 as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&String> {
        if self.char5.is_some() {
            return Ok(self.char5.as_ref().unwrap());
        }
        // pushPos(_io)
        // seek(_io, IntNum(5))
        // popPos(_io)
        return Ok(self.char5.as_ref().unwrap());
    }
    fn tuple5<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Expr2>,
        _parent: Option<TypedStack<(&'r Expr2, <Expr2 as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&Expr2_Tuple> {
        if self.tuple5.is_some() {
            return Ok(self.tuple5.as_ref().unwrap());
        }
        // pushPos(_io)
        // seek(_io, IntNum(5))
        // popPos(_io)
        return Ok(self.tuple5.as_ref().unwrap());
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct Expr2_Tuple {
    pub byte0: u8,
    pub byte1: u8,
    pub byte2: u8,
    pub avg: Option<i32>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for Expr2_Tuple {
    type Root = Expr2;
    type ParentStack = (&'r Expr2, <Expr2 as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.byte0 = _io.read_u1()?;
        self.byte1 = _io.read_u1()?;
        self.byte2 = _io.read_u1()?;
        Ok(())
    }
}
impl<'r, 's: 'r> Expr2_Tuple {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = Expr2_Tuple::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn avg<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Expr2>,
        _parent: Option<TypedStack<(&'r Expr2, <Expr2 as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.avg.is_some() {
            return Ok(self.avg.as_ref().unwrap());
        }
        self.avg = Some((self.byte1 + self.byte2) / 2 as i32);
        return Ok(self.avg.as_ref().unwrap());
    }
}
