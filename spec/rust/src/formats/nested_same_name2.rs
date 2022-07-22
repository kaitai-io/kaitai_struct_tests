// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct NestedSameName2 {
    pub version: u32,
    pub main_data: Option<NestedSameName2_Main>,
    pub dummy: Option<NestedSameName2_DummyObj>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedSameName2 {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.version = _io.read_u4le()?;
        self.main_data = Some(Self::read_into::<BytesReader, NestedSameName2_Main>(Self::read_into::<S, NestedSameName2_Main>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.dummy = Some(Self::read_into::<BytesReader, NestedSameName2_DummyObj>(Self::read_into::<S, NestedSameName2_DummyObj>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> NestedSameName2 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedSameName2::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NestedSameName2_Main {
    pub main_size: i32,
    pub foo: Option<NestedSameName2_Main_FooObj>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedSameName2_Main {
    type Root = NestedSameName2;
    type ParentStack = (&'r NestedSameName2, <NestedSameName2 as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.main_size = _io.read_s4le()?;
        self.foo = Some(Self::read_into::<BytesReader, NestedSameName2_Main_FooObj>(Self::read_into::<S, NestedSameName2_Main_FooObj>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> NestedSameName2_Main {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedSameName2_Main::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NestedSameName2_Main_FooObj {
    pub data1: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedSameName2_Main_FooObj {
    type Root = NestedSameName2;
    type ParentStack = (&'r NestedSameName2_Main, <NestedSameName2_Main as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.data1 = _io.read_bytes((_parent.peek().main_size * 2) as usize)?.to_vec();
        Ok(())
    }
}
impl<'r, 's: 'r> NestedSameName2_Main_FooObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedSameName2_Main_FooObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NestedSameName2_DummyObj {
    pub dummy_size: i32,
    pub foo: Option<NestedSameName2_DummyObj_FooObj>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedSameName2_DummyObj {
    type Root = NestedSameName2;
    type ParentStack = (&'r NestedSameName2, <NestedSameName2 as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.dummy_size = _io.read_s4le()?;
        self.foo = Some(Self::read_into::<BytesReader, NestedSameName2_DummyObj_FooObj>(Self::read_into::<S, NestedSameName2_DummyObj_FooObj>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> NestedSameName2_DummyObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedSameName2_DummyObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NestedSameName2_DummyObj_FooObj {
    pub data2: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedSameName2_DummyObj_FooObj {
    type Root = NestedSameName2;
    type ParentStack = (&'r NestedSameName2_DummyObj, <NestedSameName2_DummyObj as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.data2 = _io.read_bytes((_parent.peek().dummy_size * 2) as usize)?.to_vec();
        Ok(())
    }
}
impl<'r, 's: 'r> NestedSameName2_DummyObj_FooObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedSameName2_DummyObj_FooObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
