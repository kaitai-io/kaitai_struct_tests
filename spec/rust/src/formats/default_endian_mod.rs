// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct DefaultEndianMod {
    pub main: Option<DefaultEndianMod_MainObj>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for DefaultEndianMod {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.main = Some(Self::read_into::<BytesReader, DefaultEndianMod_MainObj>(Self::read_into::<S, DefaultEndianMod_MainObj>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> DefaultEndianMod {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = DefaultEndianMod::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct DefaultEndianMod_MainObj {
    pub one: i32,
    pub nest: Option<DefaultEndianMod_MainObj_Subnest>,
    pub nest_be: Option<DefaultEndianMod_MainObj_SubnestBe>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for DefaultEndianMod_MainObj {
    type Root = DefaultEndianMod;
    type ParentStack = (&'r DefaultEndianMod, <DefaultEndianMod as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.one = _io.read_s4le()?;
        self.nest = Some(Self::read_into::<BytesReader, DefaultEndianMod_MainObj_Subnest>(Self::read_into::<S, DefaultEndianMod_MainObj_Subnest>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.nest_be = Some(Self::read_into::<BytesReader, DefaultEndianMod_MainObj_SubnestBe>(Self::read_into::<S, DefaultEndianMod_MainObj_SubnestBe>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> DefaultEndianMod_MainObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = DefaultEndianMod_MainObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct DefaultEndianMod_MainObj_Subnest {
    pub two: i32,
}
impl<'r, 's: 'r> KStruct<'r, 's> for DefaultEndianMod_MainObj_Subnest {
    type Root = DefaultEndianMod;
    type ParentStack = (&'r DefaultEndianMod_MainObj, <DefaultEndianMod_MainObj as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.two = _io.read_s4le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> DefaultEndianMod_MainObj_Subnest {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = DefaultEndianMod_MainObj_Subnest::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct DefaultEndianMod_MainObj_SubnestBe {
    pub two: i32,
}
impl<'r, 's: 'r> KStruct<'r, 's> for DefaultEndianMod_MainObj_SubnestBe {
    type Root = DefaultEndianMod;
    type ParentStack = (&'r DefaultEndianMod_MainObj, <DefaultEndianMod_MainObj as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.two = _io.read_s4be()?;
        Ok(())
    }
}
impl<'r, 's: 'r> DefaultEndianMod_MainObj_SubnestBe {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = DefaultEndianMod_MainObj_SubnestBe::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
