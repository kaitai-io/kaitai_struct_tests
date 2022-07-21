// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
// extraAttrForIO(RawIdentifier(NamedIdentifier(blocks)), RepeatExpr(IntNum(2)))
// extraAttrForIO(RawIdentifier(NamedIdentifier(blocks)), RepeatExpr(IntNum(2)))

#[derive(Default, Debug, PartialEq)]
pub struct ProcessRepeatUsertype {
    pub blocks: Vec<ProcessRepeatUsertype_Block>,
    pub raw_blocks: Vec<Vec<u8>>,
    pub raw_raw_blocks: Vec<Vec<u8>>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ProcessRepeatUsertype {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.blocks = Vec::new();
        {
            // condRepeatExprHeader(NamedIdentifier(blocks), _io, UserTypeFromBytes(List(block),None,List(),BytesLimitType(IntNum(5),None,false,None,Some(ProcessXor(IntNum(158)))),Some(ProcessXor(IntNum(158)))), IntNum(2))
            // handleAssignmentRepeatExpr(RawIdentifier(RawIdentifier(NamedIdentifier(blocks))), _io.read_bytes(5 as usize)?)
            // attrProcess(ProcessXor(IntNum(158)), RawIdentifier(RawIdentifier(NamedIdentifier(blocks))), RawIdentifier(NamedIdentifier(blocks)), RepeatExpr(IntNum(2)))
            // handleAssignmentRepeatExpr(NamedIdentifier(blocks), &BytesReader::new(_io.read_bytes(5 as usize)?))
        }
        Ok(())
    }
}
impl<'r, 's: 'r> ProcessRepeatUsertype {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ProcessRepeatUsertype::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct ProcessRepeatUsertype_Block {
    pub a: i32,
    pub b: i8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ProcessRepeatUsertype_Block {
    type Root = ProcessRepeatUsertype;
    type ParentStack = (&'r ProcessRepeatUsertype, <ProcessRepeatUsertype as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.a = _io.read_s4le()?;
        self.b = _io.read_s1()?;
        Ok(())
    }
}
impl<'r, 's: 'r> ProcessRepeatUsertype_Block {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ProcessRepeatUsertype_Block::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
