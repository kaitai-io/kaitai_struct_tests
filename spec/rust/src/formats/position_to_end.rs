// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct PositionToEnd {
    pub index: Option<PositionToEnd_IndexObj>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for PositionToEnd {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        Ok(())
    }
}
impl<'r, 's: 'r> PositionToEnd {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = PositionToEnd::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn index<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&PositionToEnd_IndexObj> {
        if self.index.is_some() {
            return Ok(self.index.as_ref().unwrap());
        }
        // pushPos(_io)
        // seek(_io, BinOp(Attribute(Name(identifier(_io)),identifier(size)),Sub,IntNum(8)))
        // popPos(_io)
        return Ok(self.index.as_ref().unwrap());
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct PositionToEnd_IndexObj {
    pub foo: u32,
    pub bar: u32,
}
impl<'r, 's: 'r> KStruct<'r, 's> for PositionToEnd_IndexObj {
    type Root = PositionToEnd;
    type ParentStack = (&'r PositionToEnd, <PositionToEnd as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.foo = _io.read_u4le()?;
        self.bar = _io.read_u4le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> PositionToEnd_IndexObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = PositionToEnd_IndexObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
