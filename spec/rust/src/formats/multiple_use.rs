// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct MultipleUse {
    pub t1: Option<MultipleUse_Type1>,
    pub t2: Option<MultipleUse_Type2>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for MultipleUse {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.t1 = Some(Self::read_into::<BytesReader, MultipleUse_Type1>(Self::read_into::<S, MultipleUse_Type1>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.t2 = Some(Self::read_into::<BytesReader, MultipleUse_Type2>(Self::read_into::<S, MultipleUse_Type2>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> MultipleUse {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = MultipleUse::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct MultipleUse_Multi {
    pub value: i32,
}
impl<'r, 's: 'r> KStruct<'r, 's> for MultipleUse_Multi {
    type Root = MultipleUse;
    type ParentStack = (&'r MultipleUse, <MultipleUse as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.value = _io.read_s4le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> MultipleUse_Multi {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = MultipleUse_Multi::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct MultipleUse_Type1 {
    pub first_use: Option<MultipleUse_Multi>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for MultipleUse_Type1 {
    type Root = MultipleUse;
    type ParentStack = (&'r MultipleUse, <MultipleUse as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.first_use = Some(Self::read_into::<BytesReader, MultipleUse_Multi>(Self::read_into::<S, MultipleUse_Multi>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> MultipleUse_Type1 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = MultipleUse_Type1::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct MultipleUse_Type2 {
    pub second_use: Option<MultipleUse_Multi>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for MultipleUse_Type2 {
    type Root = MultipleUse;
    type ParentStack = (&'r MultipleUse, <MultipleUse as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        Ok(())
    }
}
impl<'r, 's: 'r> MultipleUse_Type2 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = MultipleUse_Type2::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn second_use<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r MultipleUse>,
        _parent: Option<TypedStack<(&'r MultipleUse, <MultipleUse as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&MultipleUse_Multi> {
        if self.second_use.is_some() {
            return Ok(self.second_use.as_ref().unwrap());
        }
        // pushPos(_io)
        // seek(_io, IntNum(0))
        // popPos(_io)
        return Ok(self.second_use.as_ref().unwrap());
    }
}
