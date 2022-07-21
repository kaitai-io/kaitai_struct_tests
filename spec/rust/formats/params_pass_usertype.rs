// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ParamsPassUsertype {
    pub first: Option<ParamsPassUsertype_Block>,
    pub one: Option<ParamsPassUsertype_ParamType>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ParamsPassUsertype {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.first = Some(Self::read_into::<BytesReader, ParamsPassUsertype_Block>(Self::read_into::<S, ParamsPassUsertype_Block>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.one = Some(Self::read_into::<BytesReader, ParamsPassUsertype_ParamType>(Self::read_into::<S, ParamsPassUsertype_ParamType>(self.first, _io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> ParamsPassUsertype {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ParamsPassUsertype::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct ParamsPassUsertype_Block {
    pub foo: u8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ParamsPassUsertype_Block {
    type Root = ParamsPassUsertype;
    type ParentStack = (&'r ParamsPassUsertype, <ParamsPassUsertype as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.foo = _io.read_u1()?;
        Ok(())
    }
}
impl<'r, 's: 'r> ParamsPassUsertype_Block {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ParamsPassUsertype_Block::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct ParamsPassUsertype_ParamType {
    pub foo: Option<ParamsPassUsertype_Block>,
    pub buf: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ParamsPassUsertype_ParamType {
    type Root = ParamsPassUsertype;
    type ParentStack = (&'r ParamsPassUsertype, <ParamsPassUsertype as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.buf = _io.read_bytes(self.foo.foo as usize)?.to_vec();
        Ok(())
    }
}
impl<'r, 's: 'r> ParamsPassUsertype_ParamType {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ParamsPassUsertype_ParamType::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
