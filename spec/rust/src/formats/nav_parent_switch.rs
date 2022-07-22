// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct NavParentSwitch {
    pub category: u8,
    pub content: Option<NavParentSwitch_Content>,
}
#[derive(Debug, PartialEq)]
pub enum NavParentSwitch_Content {
    NavParentSwitch_Element1(NavParentSwitch_Element1),
}
impl From<NavParentSwitch_Element1> for NavParentSwitch_Content {
    fn from(v: NavParentSwitch_Element1) -> Self {
        Self::NavParentSwitch_Element1(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for NavParentSwitch {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.category = _io.read_u1()?;
        match self.category {
            1 => {
                self.content = Some(Self::read_into::<S, NavParentSwitch_Element1>(_io, _root, _parent.push(self))?.into());
            }
            _ => panic!("unhandled value")
        }
        Ok(())
    }
}
impl<'r, 's: 'r> NavParentSwitch {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParentSwitch::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NavParentSwitch_Element1 {
    pub foo: u8,
    pub subelement: Option<NavParentSwitch_Subelement1>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavParentSwitch_Element1 {
    type Root = NavParentSwitch;
    type ParentStack = (&'r NavParentSwitch, <NavParentSwitch as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.foo = _io.read_u1()?;
        self.subelement = Some(Self::read_into::<BytesReader, NavParentSwitch_Subelement1>(Self::read_into::<S, NavParentSwitch_Subelement1>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> NavParentSwitch_Element1 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParentSwitch_Element1::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NavParentSwitch_Subelement1 {
    pub bar: u8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavParentSwitch_Subelement1 {
    type Root = NavParentSwitch;
    type ParentStack = (&'r NavParentSwitch, <NavParentSwitch as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        {
            // condIfHeader(Compare(Attribute(Name(identifier(_parent)),identifier(foo)),Eq,IntNum(66)))
            self.bar = _io.read_u1()?;
        }
        Ok(())
    }
}
impl<'r, 's: 'r> NavParentSwitch_Subelement1 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParentSwitch_Subelement1::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
