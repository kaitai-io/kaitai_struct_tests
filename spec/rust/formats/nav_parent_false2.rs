// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct NavParentFalse2 {
    pub parentless: Option<NavParentFalse2_Child>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavParentFalse2 {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.parentless = Some(Self::read_into::<BytesReader, NavParentFalse2_Child>(Self::read_into::<S, NavParentFalse2_Child>(_io, _root, KStructUnit::parent_stack())?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> NavParentFalse2 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParentFalse2::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NavParentFalse2_Child {
    pub foo: u8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavParentFalse2_Child {
    type Root = NavParentFalse2;
    type ParentStack = (&'r NavParentFalse2, <NavParentFalse2 as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> NavParentFalse2_Child {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParentFalse2_Child::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
