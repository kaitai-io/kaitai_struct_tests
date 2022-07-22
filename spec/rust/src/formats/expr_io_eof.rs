// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
// extraAttrForIO(RawIdentifier(NamedIdentifier(substream1)), NoRepeat)
// extraAttrForIO(RawIdentifier(NamedIdentifier(substream2)), NoRepeat)

#[derive(Default, Debug, PartialEq)]
pub struct ExprIoEof {
    pub substream1: Option<ExprIoEof_OneOrTwo>,
    pub substream2: Option<ExprIoEof_OneOrTwo>,
    pub raw_substream1: Vec<u8>,
    pub raw_substream2: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ExprIoEof {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.substream1 = Some(Self::read_into::<BytesReader, ExprIoEof_OneOrTwo>(&BytesReader::new(_io.read_bytes(4 as usize)?), Some(self), _parent.push(self))?);
        self.substream2 = Some(Self::read_into::<BytesReader, ExprIoEof_OneOrTwo>(&BytesReader::new(_io.read_bytes(8 as usize)?), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> ExprIoEof {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprIoEof::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct ExprIoEof_OneOrTwo {
    pub one: u32,
    pub two: u32,
    pub reflect_eof: Option<bool>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ExprIoEof_OneOrTwo {
    type Root = ExprIoEof;
    type ParentStack = (&'r ExprIoEof, <ExprIoEof as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.one = _io.read_u4le()?;
        {
            // condIfHeader(UnaryOp(Not,Attribute(Name(identifier(_io)),identifier(eof))))
            self.two = _io.read_u4le()?;
        }
        Ok(())
    }
}
impl<'r, 's: 'r> ExprIoEof_OneOrTwo {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprIoEof_OneOrTwo::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn reflect_eof<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r ExprIoEof>,
        _parent: Option<TypedStack<(&'r ExprIoEof, <ExprIoEof as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&bool> {
        if self.reflect_eof.is_some() {
            return Ok(self.reflect_eof.as_ref().unwrap());
        }
        self.reflect_eof = Some(_io.is_eof as bool);
        return Ok(self.reflect_eof.as_ref().unwrap());
    }
}
