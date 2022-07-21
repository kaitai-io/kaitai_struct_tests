// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct NavParentFalse {
    pub child_size: u8,
    pub element_a: Option<NavParentFalse_ParentA>,
    pub element_b: Option<NavParentFalse_ParentB>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavParentFalse {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.child_size = _io.read_u1()?;
        self.element_a = Some(Self::read_into::<BytesReader, NavParentFalse_ParentA>(Self::read_into::<S, NavParentFalse_ParentA>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.element_b = Some(Self::read_into::<BytesReader, NavParentFalse_ParentB>(Self::read_into::<S, NavParentFalse_ParentB>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> NavParentFalse {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParentFalse::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NavParentFalse_ParentA {
    pub foo: Option<NavParentFalse_Child>,
    pub bar: Option<NavParentFalse_ParentB>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavParentFalse_ParentA {
    type Root = NavParentFalse;
    type ParentStack = (&'r NavParentFalse, <NavParentFalse as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.foo = Some(Self::read_into::<BytesReader, NavParentFalse_Child>(Self::read_into::<S, NavParentFalse_Child>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.bar = Some(Self::read_into::<BytesReader, NavParentFalse_ParentB>(Self::read_into::<S, NavParentFalse_ParentB>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> NavParentFalse_ParentA {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParentFalse_ParentA::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NavParentFalse_ParentB {
    pub foo: Option<NavParentFalse_Child>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavParentFalse_ParentB {
    type Root = NavParentFalse;
    type ParentStack = (&'r NavParentFalse, <NavParentFalse as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.foo = Some(Self::read_into::<BytesReader, NavParentFalse_Child>(Self::read_into::<S, NavParentFalse_Child>(_io, _root, KStructUnit::parent_stack())?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> NavParentFalse_ParentB {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParentFalse_ParentB::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NavParentFalse_Child {
    pub code: u8,
    pub more: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavParentFalse_Child {
    type Root = NavParentFalse;
    type ParentStack = (&'r NavParentFalse, <NavParentFalse as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.code = _io.read_u1()?;
        {
            // condIfHeader(Compare(Name(identifier(code)),Eq,IntNum(73)))
            self.more = _io.read_bytes(_parent.peek()._parent.child_size as usize)?.to_vec();
        }
        Ok(())
    }
}
impl<'r, 's: 'r> NavParentFalse_Child {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParentFalse_Child::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
