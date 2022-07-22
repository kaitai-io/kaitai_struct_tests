// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct PositionInSeq {
    pub numbers: Vec<u8>,
    pub header: Option<PositionInSeq_HeaderObj>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for PositionInSeq {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.numbers = Vec::new();
        {
            // condRepeatExprHeader(NamedIdentifier(numbers), _io, Int1Type(false), Attribute(Name(identifier(header)),identifier(qty_numbers)))
            // handleAssignmentRepeatExpr(NamedIdentifier(numbers), _io.read_u1()?)
        }
        Ok(())
    }
}
impl<'r, 's: 'r> PositionInSeq {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = PositionInSeq::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn header<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&PositionInSeq_HeaderObj> {
        if self.header.is_some() {
            return Ok(self.header.as_ref().unwrap());
        }
        // pushPos(_io)
        // seek(_io, IntNum(16))
        // popPos(_io)
        return Ok(self.header.as_ref().unwrap());
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct PositionInSeq_HeaderObj {
    pub qty_numbers: u32,
}
impl<'r, 's: 'r> KStruct<'r, 's> for PositionInSeq_HeaderObj {
    type Root = PositionInSeq;
    type ParentStack = (&'r PositionInSeq, <PositionInSeq as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.qty_numbers = _io.read_u4le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> PositionInSeq_HeaderObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = PositionInSeq_HeaderObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
