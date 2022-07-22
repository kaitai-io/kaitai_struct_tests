// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct RepeatNStruct {
    pub qty: u32,
    pub chunks: Vec<RepeatNStruct_Chunk>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for RepeatNStruct {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.qty = _io.read_u4le()?;
        self.chunks = Vec::new();
        {
            // condRepeatExprHeader(NamedIdentifier(chunks), _io, UserTypeInstream(List(chunk),None,List()), Name(identifier(qty)))
            // handleAssignmentRepeatExpr(NamedIdentifier(chunks), Self::read_into::<S, RepeatNStruct_Chunk>(_io, _root, _parent.push(self))?.into())
        }
        Ok(())
    }
}
impl<'r, 's: 'r> RepeatNStruct {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = RepeatNStruct::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct RepeatNStruct_Chunk {
    pub offset: u32,
    pub len: u32,
}
impl<'r, 's: 'r> KStruct<'r, 's> for RepeatNStruct_Chunk {
    type Root = RepeatNStruct;
    type ParentStack = (&'r RepeatNStruct, <RepeatNStruct as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.offset = _io.read_u4le()?;
        self.len = _io.read_u4le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> RepeatNStruct_Chunk {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = RepeatNStruct_Chunk::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
