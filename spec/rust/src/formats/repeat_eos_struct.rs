// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct RepeatEosStruct {
    pub chunks: Vec<RepeatEosStruct_Chunk>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for RepeatEosStruct {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.chunks = Vec::new();
        {
            type ArrayElement = RepeatEosStruct_Chunk;
            while !_io.is_eof() {
                self.chunks.push(Self::read_into::<S, ArrayElement>(_io, _root, _parent.push(self))?);
            }
        }
        Ok(())
    }
}
impl<'r, 's: 'r> RepeatEosStruct {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = RepeatEosStruct::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct RepeatEosStruct_Chunk {
    pub offset: u32,
    pub len: u32,
}
impl<'r, 's: 'r> KStruct<'r, 's> for RepeatEosStruct_Chunk {
    type Root = RepeatEosStruct;
    type ParentStack = (&'r RepeatEosStruct, <RepeatEosStruct as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> RepeatEosStruct_Chunk {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = RepeatEosStruct_Chunk::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
