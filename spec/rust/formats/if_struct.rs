// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct IfStruct {
    pub op1: Option<IfStruct_Operation>,
    pub op2: Option<IfStruct_Operation>,
    pub op3: Option<IfStruct_Operation>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for IfStruct {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.op1 = Some(Self::read_into::<BytesReader, IfStruct_Operation>(Self::read_into::<S, IfStruct_Operation>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.op2 = Some(Self::read_into::<BytesReader, IfStruct_Operation>(Self::read_into::<S, IfStruct_Operation>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.op3 = Some(Self::read_into::<BytesReader, IfStruct_Operation>(Self::read_into::<S, IfStruct_Operation>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> IfStruct {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = IfStruct::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct IfStruct_Operation {
    pub opcode: u8,
    pub arg_tuple: Option<IfStruct_ArgTuple>,
    pub arg_str: Option<IfStruct_ArgStr>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for IfStruct_Operation {
    type Root = IfStruct;
    type ParentStack = (&'r IfStruct, <IfStruct as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.opcode = _io.read_u1()?;
        {
            // condIfHeader(Compare(Name(identifier(opcode)),Eq,IntNum(84)))
            self.arg_tuple = Some(Self::read_into::<BytesReader, IfStruct_ArgTuple>(Self::read_into::<S, IfStruct_ArgTuple>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        }
        {
            // condIfHeader(Compare(Name(identifier(opcode)),Eq,IntNum(83)))
            self.arg_str = Some(Self::read_into::<BytesReader, IfStruct_ArgStr>(Self::read_into::<S, IfStruct_ArgStr>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        }
        Ok(())
    }
}
impl<'r, 's: 'r> IfStruct_Operation {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = IfStruct_Operation::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct IfStruct_ArgTuple {
    pub num1: u8,
    pub num2: u8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for IfStruct_ArgTuple {
    type Root = IfStruct;
    type ParentStack = (&'r IfStruct, <IfStruct as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.num1 = _io.read_u1()?;
        self.num2 = _io.read_u1()?;
        Ok(())
    }
}
impl<'r, 's: 'r> IfStruct_ArgTuple {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = IfStruct_ArgTuple::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct IfStruct_ArgStr {
    pub len: u8,
    pub str: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for IfStruct_ArgStr {
    type Root = IfStruct;
    type ParentStack = (&'r IfStruct, <IfStruct as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.len = _io.read_u1()?;
        self.str = decode_string(_io.read_bytes(self.len as usize)?, "UTF-8")?;
        Ok(())
    }
}
impl<'r, 's: 'r> IfStruct_ArgStr {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = IfStruct_ArgStr::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
