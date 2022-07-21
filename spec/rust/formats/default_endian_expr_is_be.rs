// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct DefaultEndianExprIsBe {
    pub docs: Vec<DefaultEndianExprIsBe_Doc>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for DefaultEndianExprIsBe {
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
            type ArrayElement = DefaultEndianExprIsBe_Doc;
            while !_io.is_eof() {
                self.docs.push(Self::read_into::<S, ArrayElement>(_io, _root, _parent.push(self))?);
            }
        }
        Ok(())
    }
}
impl<'r, 's: 'r> DefaultEndianExprIsBe {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = DefaultEndianExprIsBe::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct DefaultEndianExprIsBe_Doc {
    pub indicator: Vec<u8>,
    pub main: Option<DefaultEndianExprIsBe_Doc_MainObj>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for DefaultEndianExprIsBe_Doc {
    type Root = DefaultEndianExprIsBe;
    type ParentStack = (&'r DefaultEndianExprIsBe, <DefaultEndianExprIsBe as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.indicator = _io.read_bytes(2 as usize)?.to_vec();
        self.main = Some(Self::read_into::<BytesReader, DefaultEndianExprIsBe_Doc_MainObj>(Self::read_into::<S, DefaultEndianExprIsBe_Doc_MainObj>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> DefaultEndianExprIsBe_Doc {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = DefaultEndianExprIsBe_Doc::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct DefaultEndianExprIsBe_Doc_MainObj {
    pub some_int: u32,
    pub some_int_be: u16,
    pub some_int_le: u16,
    pub inst_int: Option<u32>,
    pub inst_sub: Option<DefaultEndianExprIsBe_Doc_MainObj_SubMainObj>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for DefaultEndianExprIsBe_Doc_MainObj {
    type Root = DefaultEndianExprIsBe;
    type ParentStack = (&'r DefaultEndianExprIsBe_Doc, <DefaultEndianExprIsBe_Doc as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.some_int = panic!("Unable to parse unknown-endian integers");
        self.some_int_be = _io.read_u2be()?;
        self.some_int_le = _io.read_u2le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> DefaultEndianExprIsBe_Doc_MainObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = DefaultEndianExprIsBe_Doc_MainObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn inst_int<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r DefaultEndianExprIsBe>,
        _parent: Option<TypedStack<(&'r DefaultEndianExprIsBe_Doc, <DefaultEndianExprIsBe_Doc as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&u32> {
        if self.inst_int.is_some() {
            return Ok(self.inst_int.as_ref().unwrap());
        }
        // pushPos(_io)
        // seek(_io, IntNum(2))
        // popPos(_io)
        return Ok(self.inst_int.as_ref().unwrap());
    }
    fn inst_sub<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r DefaultEndianExprIsBe>,
        _parent: Option<TypedStack<(&'r DefaultEndianExprIsBe_Doc, <DefaultEndianExprIsBe_Doc as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&DefaultEndianExprIsBe_Doc_MainObj_SubMainObj> {
        if self.inst_sub.is_some() {
            return Ok(self.inst_sub.as_ref().unwrap());
        }
        // pushPos(_io)
        // seek(_io, IntNum(2))
        // popPos(_io)
        return Ok(self.inst_sub.as_ref().unwrap());
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct DefaultEndianExprIsBe_Doc_MainObj_SubMainObj {
    pub foo: u32,
}
impl<'r, 's: 'r> KStruct<'r, 's> for DefaultEndianExprIsBe_Doc_MainObj_SubMainObj {
    type Root = DefaultEndianExprIsBe;
    type ParentStack = (&'r DefaultEndianExprIsBe_Doc_MainObj, <DefaultEndianExprIsBe_Doc_MainObj as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.foo = panic!("Unable to parse unknown-endian integers");
        Ok(())
    }
}
impl<'r, 's: 'r> DefaultEndianExprIsBe_Doc_MainObj_SubMainObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = DefaultEndianExprIsBe_Doc_MainObj_SubMainObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
