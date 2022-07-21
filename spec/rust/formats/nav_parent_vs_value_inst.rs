// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct NavParentVsValueInst {
    pub s1: String,
    pub child: Option<NavParentVsValueInst_ChildObj>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavParentVsValueInst {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.s1 = decode_string(_io.read_bytes_term(124, false, true, true)?, "UTF-8")?;
        self.child = Some(Self::read_into::<BytesReader, NavParentVsValueInst_ChildObj>(Self::read_into::<S, NavParentVsValueInst_ChildObj>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> NavParentVsValueInst {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParentVsValueInst::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NavParentVsValueInst_ChildObj {
    pub do_something: Option<bool>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavParentVsValueInst_ChildObj {
    type Root = NavParentVsValueInst;
    type ParentStack = (&'r NavParentVsValueInst, <NavParentVsValueInst as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        Ok(())
    }
}
impl<'r, 's: 'r> NavParentVsValueInst_ChildObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParentVsValueInst_ChildObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn do_something<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r NavParentVsValueInst>,
        _parent: Option<TypedStack<(&'r NavParentVsValueInst, <NavParentVsValueInst as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&bool> {
        if self.do_something.is_some() {
            return Ok(self.do_something.as_ref().unwrap());
        }
        self.do_something = Some(if _parent.peek().s1 == "foo" { true } else { false} as bool);
        return Ok(self.do_something.as_ref().unwrap());
    }
}
