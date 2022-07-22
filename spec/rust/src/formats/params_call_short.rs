// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ParamsCallShort {
    pub buf1: Option<ParamsCallShort_MyStr1>,
    pub buf2: Option<ParamsCallShort_MyStr2>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ParamsCallShort {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.buf1 = Some(Self::read_into::<BytesReader, ParamsCallShort_MyStr1>(Self::read_into::<S, ParamsCallShort_MyStr1>(5, _io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.buf2 = Some(Self::read_into::<BytesReader, ParamsCallShort_MyStr2>(Self::read_into::<S, ParamsCallShort_MyStr2>((2 + 3), true, _io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> ParamsCallShort {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ParamsCallShort::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct ParamsCallShort_MyStr1 {
    pub len: u32,
    pub body: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ParamsCallShort_MyStr1 {
    type Root = ParamsCallShort;
    type ParentStack = (&'r ParamsCallShort, <ParamsCallShort as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.body = decode_string(_io.read_bytes(self.len as usize)?, "UTF-8")?;
        Ok(())
    }
}
impl<'r, 's: 'r> ParamsCallShort_MyStr1 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ParamsCallShort_MyStr1::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct ParamsCallShort_MyStr2 {
    pub len: u32,
    pub has_trailer: bool,
    pub body: String,
    pub trailer: u8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ParamsCallShort_MyStr2 {
    type Root = ParamsCallShort;
    type ParentStack = (&'r ParamsCallShort, <ParamsCallShort as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.body = decode_string(_io.read_bytes(self.len as usize)?, "UTF-8")?;
        {
            // condIfHeader(Name(identifier(has_trailer)))
            self.trailer = _io.read_u1()?;
        }
        Ok(())
    }
}
impl<'r, 's: 'r> ParamsCallShort_MyStr2 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ParamsCallShort_MyStr2::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
