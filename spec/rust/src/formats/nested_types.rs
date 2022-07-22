// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct NestedTypes {
    pub one: Option<NestedTypes_SubtypeA>,
    pub two: Option<NestedTypes_SubtypeB>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedTypes {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.one = Some(Self::read_into::<BytesReader, NestedTypes_SubtypeA>(Self::read_into::<S, NestedTypes_SubtypeA>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.two = Some(Self::read_into::<BytesReader, NestedTypes_SubtypeB>(Self::read_into::<S, NestedTypes_SubtypeB>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> NestedTypes {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedTypes::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NestedTypes_SubtypeA {
    pub typed_at_root: Option<NestedTypes_SubtypeB>,
    pub typed_here: Option<NestedTypes_SubtypeA_SubtypeC>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedTypes_SubtypeA {
    type Root = NestedTypes;
    type ParentStack = (&'r NestedTypes, <NestedTypes as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.typed_at_root = Some(Self::read_into::<BytesReader, NestedTypes_SubtypeB>(Self::read_into::<S, NestedTypes_SubtypeB>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.typed_here = Some(Self::read_into::<BytesReader, NestedTypes_SubtypeA_SubtypeC>(Self::read_into::<S, NestedTypes_SubtypeA_SubtypeC>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> NestedTypes_SubtypeA {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedTypes_SubtypeA::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NestedTypes_SubtypeA_SubtypeC {
    pub value_c: i8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedTypes_SubtypeA_SubtypeC {
    type Root = NestedTypes;
    type ParentStack = (&'r NestedTypes_SubtypeA, <NestedTypes_SubtypeA as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.value_c = _io.read_s1()?;
        Ok(())
    }
}
impl<'r, 's: 'r> NestedTypes_SubtypeA_SubtypeC {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedTypes_SubtypeA_SubtypeC::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NestedTypes_SubtypeB {
    pub value_b: i8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedTypes_SubtypeB {
    type Root = NestedTypes;
    type ParentStack = (&'r NestedTypes, <NestedTypes as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.value_b = _io.read_s1()?;
        Ok(())
    }
}
impl<'r, 's: 'r> NestedTypes_SubtypeB {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedTypes_SubtypeB::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
