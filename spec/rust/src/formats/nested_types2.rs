// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct NestedTypes2 {
    pub one: Option<NestedTypes2_SubtypeA>,
    pub two: Option<NestedTypes2_SubtypeB>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedTypes2 {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.one = Some(Self::read_into::<BytesReader, NestedTypes2_SubtypeA>(Self::read_into::<S, NestedTypes2_SubtypeA>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.two = Some(Self::read_into::<BytesReader, NestedTypes2_SubtypeB>(Self::read_into::<S, NestedTypes2_SubtypeB>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> NestedTypes2 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedTypes2::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NestedTypes2_SubtypeA {
    pub typed_at_root: Option<NestedTypes2_SubtypeB>,
    pub typed_here1: Option<NestedTypes2_SubtypeA_SubtypeC>,
    pub typed_here2: Option<NestedTypes2_SubtypeA_SubtypeCc>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedTypes2_SubtypeA {
    type Root = NestedTypes2;
    type ParentStack = (&'r NestedTypes2, <NestedTypes2 as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.typed_at_root = Some(Self::read_into::<BytesReader, NestedTypes2_SubtypeB>(Self::read_into::<S, NestedTypes2_SubtypeB>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.typed_here1 = Some(Self::read_into::<BytesReader, NestedTypes2_SubtypeA_SubtypeC>(Self::read_into::<S, NestedTypes2_SubtypeA_SubtypeC>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.typed_here2 = Some(Self::read_into::<BytesReader, NestedTypes2_SubtypeA_SubtypeCc>(Self::read_into::<S, NestedTypes2_SubtypeA_SubtypeCc>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> NestedTypes2_SubtypeA {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedTypes2_SubtypeA::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NestedTypes2_SubtypeA_SubtypeC {
    pub value_c: i8,
    pub typed_here: Option<NestedTypes2_SubtypeA_SubtypeC_SubtypeD>,
    pub typed_parent: Option<NestedTypes2_SubtypeA_SubtypeCc>,
    pub typed_root: Option<NestedTypes2_SubtypeB>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedTypes2_SubtypeA_SubtypeC {
    type Root = NestedTypes2;
    type ParentStack = (&'r NestedTypes2_SubtypeA, <NestedTypes2_SubtypeA as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.value_c = _io.read_s1()?;
        self.typed_here = Some(Self::read_into::<BytesReader, NestedTypes2_SubtypeA_SubtypeC_SubtypeD>(Self::read_into::<S, NestedTypes2_SubtypeA_SubtypeC_SubtypeD>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.typed_parent = Some(Self::read_into::<BytesReader, NestedTypes2_SubtypeA_SubtypeCc>(Self::read_into::<S, NestedTypes2_SubtypeA_SubtypeCc>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.typed_root = Some(Self::read_into::<BytesReader, NestedTypes2_SubtypeB>(Self::read_into::<S, NestedTypes2_SubtypeB>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> NestedTypes2_SubtypeA_SubtypeC {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedTypes2_SubtypeA_SubtypeC::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NestedTypes2_SubtypeA_SubtypeC_SubtypeD {
    pub value_d: i8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedTypes2_SubtypeA_SubtypeC_SubtypeD {
    type Root = NestedTypes2;
    type ParentStack = (&'r NestedTypes2_SubtypeA_SubtypeC, <NestedTypes2_SubtypeA_SubtypeC as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.value_d = _io.read_s1()?;
        Ok(())
    }
}
impl<'r, 's: 'r> NestedTypes2_SubtypeA_SubtypeC_SubtypeD {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedTypes2_SubtypeA_SubtypeC_SubtypeD::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NestedTypes2_SubtypeA_SubtypeCc {
    pub value_cc: i8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedTypes2_SubtypeA_SubtypeCc {
    type Root = NestedTypes2;
    type ParentStack = (&'r NestedTypes2_SubtypeA, <NestedTypes2_SubtypeA as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.value_cc = _io.read_s1()?;
        Ok(())
    }
}
impl<'r, 's: 'r> NestedTypes2_SubtypeA_SubtypeCc {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedTypes2_SubtypeA_SubtypeCc::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NestedTypes2_SubtypeB {
    pub value_b: i8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedTypes2_SubtypeB {
    type Root = NestedTypes2;
    type ParentStack = (&'r NestedTypes2, <NestedTypes2 as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> NestedTypes2_SubtypeB {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedTypes2_SubtypeB::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
