// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ExprSizeofType1 {
    pub sizeof_block: Option<i32>,
    pub sizeof_subblock: Option<i32>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ExprSizeofType1 {
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
impl<'r, 's: 'r> ExprSizeofType1 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprSizeofType1::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn sizeof_block<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.sizeof_block.is_some() {
            return Ok(self.sizeof_block.as_ref().unwrap());
        }
        self.sizeof_block = Some(11 as i32);
        return Ok(self.sizeof_block.as_ref().unwrap());
    }
    fn sizeof_subblock<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.sizeof_subblock.is_some() {
            return Ok(self.sizeof_subblock.as_ref().unwrap());
        }
        self.sizeof_subblock = Some(4 as i32);
        return Ok(self.sizeof_subblock.as_ref().unwrap());
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct ExprSizeofType1_Block {
    pub a: u8,
    pub b: u32,
    pub c: Vec<u8>,
    pub d: Option<ExprSizeofType1_Block_Subblock>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ExprSizeofType1_Block {
    type Root = ExprSizeofType1;
    type ParentStack = (&'r ExprSizeofType1, <ExprSizeofType1 as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.a = _io.read_u1()?;
        self.b = _io.read_u4le()?;
        self.c = _io.read_bytes(2 as usize)?.to_vec();
        self.d = Some(Self::read_into::<BytesReader, ExprSizeofType1_Block_Subblock>(Self::read_into::<S, ExprSizeofType1_Block_Subblock>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> ExprSizeofType1_Block {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprSizeofType1_Block::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct ExprSizeofType1_Block_Subblock {
    pub a: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ExprSizeofType1_Block_Subblock {
    type Root = ExprSizeofType1;
    type ParentStack = (&'r ExprSizeofType1_Block, <ExprSizeofType1_Block as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.a = _io.read_bytes(4 as usize)?.to_vec();
        Ok(())
    }
}
impl<'r, 's: 'r> ExprSizeofType1_Block_Subblock {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprSizeofType1_Block_Subblock::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
