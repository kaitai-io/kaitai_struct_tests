// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct NavParentOverride {
    pub child_size: u8,
    pub child_1: Option<NavParentOverride_Child>,
    pub mediator_2: Option<NavParentOverride_Mediator>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavParentOverride {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.child_size = _io.read_u1()?;
        self.child_1 = Some(Self::read_into::<BytesReader, NavParentOverride_Child>(Self::read_into::<S, NavParentOverride_Child>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.mediator_2 = Some(Self::read_into::<BytesReader, NavParentOverride_Mediator>(Self::read_into::<S, NavParentOverride_Mediator>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> NavParentOverride {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParentOverride::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NavParentOverride_Mediator {
    pub child_2: Option<NavParentOverride_Child>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavParentOverride_Mediator {
    type Root = NavParentOverride;
    type ParentStack = (&'r NavParentOverride, <NavParentOverride as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.child_2 = Some(Self::read_into::<BytesReader, NavParentOverride_Child>(Self::read_into::<S, NavParentOverride_Child>(_io, _root, _parent.peek())?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> NavParentOverride_Mediator {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParentOverride_Mediator::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NavParentOverride_Child {
    pub data: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavParentOverride_Child {
    type Root = NavParentOverride;
    type ParentStack = (&'r NavParentOverride, <NavParentOverride as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.data = _io.read_bytes(_parent.peek().child_size as usize)?.to_vec();
        Ok(())
    }
}
impl<'r, 's: 'r> NavParentOverride_Child {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParentOverride_Child::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
