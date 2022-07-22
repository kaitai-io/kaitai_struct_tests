// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct DefaultEndianExprIsLe {
    pub docs: Vec<DefaultEndianExprIsLe_Doc>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for DefaultEndianExprIsLe {
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
            type ArrayElement = DefaultEndianExprIsLe_Doc;
            while !_io.is_eof() {
                self.docs.push(Self::read_into::<S, ArrayElement>(_io, _root, _parent.push(self))?);
            }
        }
        Ok(())
    }
}
impl<'r, 's: 'r> DefaultEndianExprIsLe {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = DefaultEndianExprIsLe::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct DefaultEndianExprIsLe_Doc {
    pub indicator: Vec<u8>,
    pub main: Option<DefaultEndianExprIsLe_Doc_MainObj>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for DefaultEndianExprIsLe_Doc {
    type Root = DefaultEndianExprIsLe;
    type ParentStack = (&'r DefaultEndianExprIsLe, <DefaultEndianExprIsLe as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.indicator = _io.read_bytes(2 as usize)?.to_vec();
        self.main = Some(Self::read_into::<BytesReader, DefaultEndianExprIsLe_Doc_MainObj>(Self::read_into::<S, DefaultEndianExprIsLe_Doc_MainObj>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> DefaultEndianExprIsLe_Doc {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = DefaultEndianExprIsLe_Doc::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct DefaultEndianExprIsLe_Doc_MainObj {
    pub some_int: u32,
    pub some_int_be: u16,
    pub some_int_le: u16,
}
impl<'r, 's: 'r> KStruct<'r, 's> for DefaultEndianExprIsLe_Doc_MainObj {
    type Root = DefaultEndianExprIsLe;
    type ParentStack = (&'r DefaultEndianExprIsLe_Doc, <DefaultEndianExprIsLe_Doc as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> DefaultEndianExprIsLe_Doc_MainObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = DefaultEndianExprIsLe_Doc_MainObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
