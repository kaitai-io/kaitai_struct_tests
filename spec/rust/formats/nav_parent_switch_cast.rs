// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct NavParentSwitchCast {
    pub main: Option<NavParentSwitchCast_Foo>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavParentSwitchCast {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.main = Some(Self::read_into::<BytesReader, NavParentSwitchCast_Foo>(Self::read_into::<S, NavParentSwitchCast_Foo>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> NavParentSwitchCast {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParentSwitchCast::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
// extraAttrForIO(RawIdentifier(NamedIdentifier(buf)), NoRepeat)
// extraAttrForIO(RawIdentifier(NamedIdentifier(buf)), NoRepeat)

#[derive(Default, Debug, PartialEq)]
pub struct NavParentSwitchCast_Foo {
    pub buf_type: u8,
    pub flag: u8,
    pub buf: Option<NavParentSwitchCast_Foo_Buf>,
    pub raw_buf: Vec<u8>,
}
#[derive(Debug, PartialEq)]
pub enum NavParentSwitchCast_Foo_Buf {
    NavParentSwitchCast_Foo_Zero(NavParentSwitchCast_Foo_Zero),
    NavParentSwitchCast_Foo_One(NavParentSwitchCast_Foo_One),
    Bytes(Vec<u8>),
}
impl From<NavParentSwitchCast_Foo_Zero> for NavParentSwitchCast_Foo_Buf {
    fn from(v: NavParentSwitchCast_Foo_Zero) -> Self {
        Self::NavParentSwitchCast_Foo_Zero(v)
    }
}
impl From<NavParentSwitchCast_Foo_One> for NavParentSwitchCast_Foo_Buf {
    fn from(v: NavParentSwitchCast_Foo_One) -> Self {
        Self::NavParentSwitchCast_Foo_One(v)
    }
}
impl From<Vec<u8>> for NavParentSwitchCast_Foo_Buf {
    fn from(v: Vec<u8>) -> Self {
        Self::Bytes(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for NavParentSwitchCast_Foo {
    type Root = NavParentSwitchCast;
    type ParentStack = (&'r NavParentSwitchCast, <NavParentSwitchCast as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.buf_type = _io.read_u1()?;
        self.flag = _io.read_u1()?;
        match self.buf_type {
            0 => {
                self.buf = Some(&BytesReader::new(_io.read_bytes(4 as usize)?));
            }
            1 => {
                self.buf = Some(&BytesReader::new(_io.read_bytes(4 as usize)?));
            }
            // switchElseStart()
            self.buf = Some(_io.read_bytes(4 as usize)?);
        }
        _ => panic!("unhandled value")
    }
    Ok(())
}
}
impl<'r, 's: 'r> NavParentSwitchCast_Foo {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParentSwitchCast_Foo::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NavParentSwitchCast_Foo_Zero {
pub branch: Option<NavParentSwitchCast_Foo_Common>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavParentSwitchCast_Foo_Zero {
type Root = NavParentSwitchCast;
type ParentStack = (&'r NavParentSwitchCast_Foo, <NavParentSwitchCast_Foo as KStruct<'r, 's>>::ParentStack);

fn read<S: KStream>(
    &mut self,
    _io: &'s S,
    _root: Option<&'r Self::Root>,
    _parent: Option<TypedStack<Self::ParentStack>>
) -> KResult<()> {
    self.branch = Some(Self::read_into::<BytesReader, NavParentSwitchCast_Foo_Common>(Self::read_into::<S, NavParentSwitchCast_Foo_Common>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
    Ok(())
}
}
impl<'r, 's: 'r> NavParentSwitchCast_Foo_Zero {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParentSwitchCast_Foo_Zero::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NavParentSwitchCast_Foo_One {
pub branch: Option<NavParentSwitchCast_Foo_Common>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavParentSwitchCast_Foo_One {
type Root = NavParentSwitchCast;
type ParentStack = (&'r NavParentSwitchCast_Foo, <NavParentSwitchCast_Foo as KStruct<'r, 's>>::ParentStack);

fn read<S: KStream>(
    &mut self,
    _io: &'s S,
    _root: Option<&'r Self::Root>,
    _parent: Option<TypedStack<Self::ParentStack>>
) -> KResult<()> {
    self.branch = Some(Self::read_into::<BytesReader, NavParentSwitchCast_Foo_Common>(Self::read_into::<S, NavParentSwitchCast_Foo_Common>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
    Ok(())
}
}
impl<'r, 's: 'r> NavParentSwitchCast_Foo_One {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParentSwitchCast_Foo_One::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NavParentSwitchCast_Foo_Common {
pub flag: Option<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavParentSwitchCast_Foo_Common {
type Root = NavParentSwitchCast;
type ParentStack = (&'r NavParentSwitchCast_Foo, <NavParentSwitchCast_Foo as KStruct<'r, 's>>::ParentStack);

fn read<S: KStream>(
    &mut self,
    _io: &'s S,
    _root: Option<&'r Self::Root>,
    _parent: Option<TypedStack<Self::ParentStack>>
) -> KResult<()> {
    Ok(())
}
}
impl<'r, 's: 'r> NavParentSwitchCast_Foo_Common {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParentSwitchCast_Foo_Common::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

fn flag<S: KStream>(
    &mut self,
    _io: &'s S,
    _root: Option<&'r NavParentSwitchCast>,
    _parent: Option<TypedStack<(&'r NavParentSwitchCast_Foo, <NavParentSwitchCast_Foo as KStruct<'r, 's>>::ParentStack)>>
) -> KResult<&u8> {
    if self.flag.is_some() {
        return Ok(self.flag.as_ref().unwrap());
    }
    self.flag = Some(_parent.peek()._parent.flag as u8);
    return Ok(self.flag.as_ref().unwrap());
}
}
