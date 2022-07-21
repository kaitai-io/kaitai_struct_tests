// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ParamsCallExtraParens {
    pub buf1: Option<ParamsCallExtraParens_MyStr1>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ParamsCallExtraParens {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.buf1 = Some(Self::read_into::<BytesReader, ParamsCallExtraParens_MyStr1>(Self::read_into::<S, ParamsCallExtraParens_MyStr1>(5, _io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> ParamsCallExtraParens {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ParamsCallExtraParens::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct ParamsCallExtraParens_MyStr1 {
    pub len: u32,
    pub body: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ParamsCallExtraParens_MyStr1 {
    type Root = ParamsCallExtraParens;
    type ParentStack = (&'r ParamsCallExtraParens, <ParamsCallExtraParens as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> ParamsCallExtraParens_MyStr1 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ParamsCallExtraParens_MyStr1::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
