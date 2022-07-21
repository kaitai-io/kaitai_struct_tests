// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct NestedSameName {
    pub main_data: Option<NestedSameName_Main>,
    pub dummy: Option<NestedSameName_DummyObj>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedSameName {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.main_data = Some(Self::read_into::<BytesReader, NestedSameName_Main>(Self::read_into::<S, NestedSameName_Main>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.dummy = Some(Self::read_into::<BytesReader, NestedSameName_DummyObj>(Self::read_into::<S, NestedSameName_DummyObj>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> NestedSameName {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedSameName::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NestedSameName_Main {
    pub main_size: i32,
    pub foo: Option<NestedSameName_Main_FooObj>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedSameName_Main {
    type Root = NestedSameName;
    type ParentStack = (&'r NestedSameName, <NestedSameName as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.main_size = _io.read_s4le()?;
        self.foo = Some(Self::read_into::<BytesReader, NestedSameName_Main_FooObj>(Self::read_into::<S, NestedSameName_Main_FooObj>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> NestedSameName_Main {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedSameName_Main::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NestedSameName_Main_FooObj {
    pub data: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedSameName_Main_FooObj {
    type Root = NestedSameName;
    type ParentStack = (&'r NestedSameName_Main, <NestedSameName_Main as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.data = _io.read_bytes((_parent.peek().main_size * 2) as usize)?.to_vec();
        Ok(())
    }
}
impl<'r, 's: 'r> NestedSameName_Main_FooObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedSameName_Main_FooObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NestedSameName_DummyObj {
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedSameName_DummyObj {
    type Root = NestedSameName;
    type ParentStack = (&'r NestedSameName, <NestedSameName as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        Ok(())
    }
}
impl<'r, 's: 'r> NestedSameName_DummyObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedSameName_DummyObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NestedSameName_DummyObj_Foo {
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedSameName_DummyObj_Foo {
    type Root = NestedSameName;
    type ParentStack = (&'r NestedSameName_DummyObj, <NestedSameName_DummyObj as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        Ok(())
    }
}
impl<'r, 's: 'r> NestedSameName_DummyObj_Foo {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedSameName_DummyObj_Foo::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
