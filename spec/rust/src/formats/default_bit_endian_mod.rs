// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct DefaultBitEndianMod {
    pub main: Option<DefaultBitEndianMod_MainObj>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for DefaultBitEndianMod {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.main = Some(Self::read_into::<BytesReader, DefaultBitEndianMod_MainObj>(Self::read_into::<S, DefaultBitEndianMod_MainObj>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> DefaultBitEndianMod {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = DefaultBitEndianMod::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct DefaultBitEndianMod_MainObj {
    pub one: u64,
    pub two: u64,
    pub nest: Option<DefaultBitEndianMod_MainObj_Subnest>,
    pub nest_be: Option<DefaultBitEndianMod_MainObj_SubnestBe>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for DefaultBitEndianMod_MainObj {
    type Root = DefaultBitEndianMod;
    type ParentStack = (&'r DefaultBitEndianMod, <DefaultBitEndianMod as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.one = _io.read_bits_int(9)?;
        self.two = _io.read_bits_int(15)?;
        _io.align_to_byte()?;
        self.nest = Some(Self::read_into::<BytesReader, DefaultBitEndianMod_MainObj_Subnest>(Self::read_into::<S, DefaultBitEndianMod_MainObj_Subnest>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.nest_be = Some(Self::read_into::<BytesReader, DefaultBitEndianMod_MainObj_SubnestBe>(Self::read_into::<S, DefaultBitEndianMod_MainObj_SubnestBe>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> DefaultBitEndianMod_MainObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = DefaultBitEndianMod_MainObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct DefaultBitEndianMod_MainObj_Subnest {
    pub two: u64,
}
impl<'r, 's: 'r> KStruct<'r, 's> for DefaultBitEndianMod_MainObj_Subnest {
    type Root = DefaultBitEndianMod;
    type ParentStack = (&'r DefaultBitEndianMod_MainObj, <DefaultBitEndianMod_MainObj as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.two = _io.read_bits_int(16)?;
        Ok(())
    }
}
impl<'r, 's: 'r> DefaultBitEndianMod_MainObj_Subnest {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = DefaultBitEndianMod_MainObj_Subnest::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct DefaultBitEndianMod_MainObj_SubnestBe {
    pub two: u64,
}
impl<'r, 's: 'r> KStruct<'r, 's> for DefaultBitEndianMod_MainObj_SubnestBe {
    type Root = DefaultBitEndianMod;
    type ParentStack = (&'r DefaultBitEndianMod_MainObj, <DefaultBitEndianMod_MainObj as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.two = _io.read_bits_int(16)?;
        Ok(())
    }
}
impl<'r, 's: 'r> DefaultBitEndianMod_MainObj_SubnestBe {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = DefaultBitEndianMod_MainObj_SubnestBe::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
