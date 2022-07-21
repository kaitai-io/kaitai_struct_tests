// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct IntegersMinMax {
    pub unsigned_min: Option<IntegersMinMax_Unsigned>,
    pub unsigned_max: Option<IntegersMinMax_Unsigned>,
    pub signed_min: Option<IntegersMinMax_Signed>,
    pub signed_max: Option<IntegersMinMax_Signed>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for IntegersMinMax {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.unsigned_min = Some(Self::read_into::<BytesReader, IntegersMinMax_Unsigned>(Self::read_into::<S, IntegersMinMax_Unsigned>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.unsigned_max = Some(Self::read_into::<BytesReader, IntegersMinMax_Unsigned>(Self::read_into::<S, IntegersMinMax_Unsigned>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.signed_min = Some(Self::read_into::<BytesReader, IntegersMinMax_Signed>(Self::read_into::<S, IntegersMinMax_Signed>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.signed_max = Some(Self::read_into::<BytesReader, IntegersMinMax_Signed>(Self::read_into::<S, IntegersMinMax_Signed>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> IntegersMinMax {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = IntegersMinMax::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct IntegersMinMax_Unsigned {
    pub u1: u8,
    pub u2le: u16,
    pub u4le: u32,
    pub u8le: u64,
    pub u2be: u16,
    pub u4be: u32,
    pub u8be: u64,
}
impl<'r, 's: 'r> KStruct<'r, 's> for IntegersMinMax_Unsigned {
    type Root = IntegersMinMax;
    type ParentStack = (&'r IntegersMinMax, <IntegersMinMax as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.u1 = _io.read_u1()?;
        self.u2le = _io.read_u2le()?;
        self.u4le = _io.read_u4le()?;
        self.u8le = _io.read_u8le()?;
        self.u2be = _io.read_u2be()?;
        self.u4be = _io.read_u4be()?;
        self.u8be = _io.read_u8be()?;
        Ok(())
    }
}
impl<'r, 's: 'r> IntegersMinMax_Unsigned {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = IntegersMinMax_Unsigned::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct IntegersMinMax_Signed {
    pub s1: i8,
    pub s2le: i16,
    pub s4le: i32,
    pub s8le: i64,
    pub s2be: i16,
    pub s4be: i32,
    pub s8be: i64,
}
impl<'r, 's: 'r> KStruct<'r, 's> for IntegersMinMax_Signed {
    type Root = IntegersMinMax;
    type ParentStack = (&'r IntegersMinMax, <IntegersMinMax as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.s1 = _io.read_s1()?;
        self.s2le = _io.read_s2le()?;
        self.s4le = _io.read_s4le()?;
        self.s8le = _io.read_s8le()?;
        self.s2be = _io.read_s2be()?;
        self.s4be = _io.read_s4be()?;
        self.s8be = _io.read_s8be()?;
        Ok(())
    }
}
impl<'r, 's: 'r> IntegersMinMax_Signed {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = IntegersMinMax_Signed::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
