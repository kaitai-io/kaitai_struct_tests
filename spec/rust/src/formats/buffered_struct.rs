// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
// extraAttrForIO(RawIdentifier(NamedIdentifier(block1)), NoRepeat)
// extraAttrForIO(RawIdentifier(NamedIdentifier(block2)), NoRepeat)

#[derive(Default, Debug, PartialEq)]
pub struct BufferedStruct {
    pub len1: u32,
    pub block1: Option<BufferedStruct_Block>,
    pub len2: u32,
    pub block2: Option<BufferedStruct_Block>,
    pub finisher: u32,
    pub raw_block1: Vec<u8>,
    pub raw_block2: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for BufferedStruct {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.len1 = _io.read_u4le()?;
        self.block1 = Some(Self::read_into::<BytesReader, BufferedStruct_Block>(&BytesReader::new(_io.read_bytes(self.len1 as usize)?), Some(self), _parent.push(self))?);
        self.len2 = _io.read_u4le()?;
        self.block2 = Some(Self::read_into::<BytesReader, BufferedStruct_Block>(&BytesReader::new(_io.read_bytes(self.len2 as usize)?), Some(self), _parent.push(self))?);
        self.finisher = _io.read_u4le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> BufferedStruct {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = BufferedStruct::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct BufferedStruct_Block {
    pub number1: u32,
    pub number2: u32,
}
impl<'r, 's: 'r> KStruct<'r, 's> for BufferedStruct_Block {
    type Root = BufferedStruct;
    type ParentStack = (&'r BufferedStruct, <BufferedStruct as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.number1 = _io.read_u4le()?;
        self.number2 = _io.read_u4le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> BufferedStruct_Block {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = BufferedStruct_Block::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
