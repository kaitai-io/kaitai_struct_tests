// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct DefaultEndianExprInherited {
    pub docs: Vec<DefaultEndianExprInherited_Doc>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for DefaultEndianExprInherited {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.docs = Vec::new();
        {
            type ArrayElement = DefaultEndianExprInherited_Doc;
            while !_io.is_eof() {
                self.docs.push(Self::read_into::<S, ArrayElement>(_io, _root, _parent.push(self))?);
            }
        }
        Ok(())
    }
}
impl<'r, 's: 'r> DefaultEndianExprInherited {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = DefaultEndianExprInherited::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct DefaultEndianExprInherited_Doc {
    pub indicator: Vec<u8>,
    pub main: Option<DefaultEndianExprInherited_Doc_MainObj>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for DefaultEndianExprInherited_Doc {
    type Root = DefaultEndianExprInherited;
    type ParentStack = (&'r DefaultEndianExprInherited, <DefaultEndianExprInherited as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.indicator = _io.read_bytes(2 as usize)?.to_vec();
        self.main = Some(Self::read_into::<BytesReader, DefaultEndianExprInherited_Doc_MainObj>(Self::read_into::<S, DefaultEndianExprInherited_Doc_MainObj>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> DefaultEndianExprInherited_Doc {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = DefaultEndianExprInherited_Doc::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct DefaultEndianExprInherited_Doc_MainObj {
    pub insides: Option<DefaultEndianExprInherited_Doc_MainObj_SubObj>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for DefaultEndianExprInherited_Doc_MainObj {
    type Root = DefaultEndianExprInherited;
    type ParentStack = (&'r DefaultEndianExprInherited_Doc, <DefaultEndianExprInherited_Doc as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.insides = Some(Self::read_into::<BytesReader, DefaultEndianExprInherited_Doc_MainObj_SubObj>(Self::read_into::<S, DefaultEndianExprInherited_Doc_MainObj_SubObj>(_io, _root, _parent.push(self), self._is_le)?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> DefaultEndianExprInherited_Doc_MainObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = DefaultEndianExprInherited_Doc_MainObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct DefaultEndianExprInherited_Doc_MainObj_SubObj {
    pub some_int: u32,
    pub more: Option<DefaultEndianExprInherited_Doc_MainObj_SubObj_SubsubObj>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for DefaultEndianExprInherited_Doc_MainObj_SubObj {
    type Root = DefaultEndianExprInherited;
    type ParentStack = (&'r DefaultEndianExprInherited_Doc_MainObj, <DefaultEndianExprInherited_Doc_MainObj as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.some_int = panic!("Unable to parse unknown-endian integers");
        self.more = Some(Self::read_into::<BytesReader, DefaultEndianExprInherited_Doc_MainObj_SubObj_SubsubObj>(Self::read_into::<S, DefaultEndianExprInherited_Doc_MainObj_SubObj_SubsubObj>(_io, _root, _parent.push(self), self._is_le)?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> DefaultEndianExprInherited_Doc_MainObj_SubObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = DefaultEndianExprInherited_Doc_MainObj_SubObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct DefaultEndianExprInherited_Doc_MainObj_SubObj_SubsubObj {
    pub some_int1: u16,
    pub some_int2: u16,
    pub some_inst: Option<u32>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for DefaultEndianExprInherited_Doc_MainObj_SubObj_SubsubObj {
    type Root = DefaultEndianExprInherited;
    type ParentStack = (&'r DefaultEndianExprInherited_Doc_MainObj_SubObj, <DefaultEndianExprInherited_Doc_MainObj_SubObj as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.some_int1 = panic!("Unable to parse unknown-endian integers");
        self.some_int2 = panic!("Unable to parse unknown-endian integers");
        Ok(())
    }
}
impl<'r, 's: 'r> DefaultEndianExprInherited_Doc_MainObj_SubObj_SubsubObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = DefaultEndianExprInherited_Doc_MainObj_SubObj_SubsubObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn some_inst<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r DefaultEndianExprInherited>,
        _parent: Option<TypedStack<(&'r DefaultEndianExprInherited_Doc_MainObj_SubObj, <DefaultEndianExprInherited_Doc_MainObj_SubObj as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&u32> {
        if self.some_inst.is_some() {
            return Ok(self.some_inst.as_ref().unwrap());
        }
        // pushPos(_io)
        // seek(_io, IntNum(2))
        // popPos(_io)
        return Ok(self.some_inst.as_ref().unwrap());
    }
}
