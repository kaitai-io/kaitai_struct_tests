// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ExprSizeofType0 {
    pub sizeof_block: Option<i32>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ExprSizeofType0 {
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
impl<'r, 's: 'r> ExprSizeofType0 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprSizeofType0::default();

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
        self.sizeof_block = Some(7 as i32);
        return Ok(self.sizeof_block.as_ref().unwrap());
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct ExprSizeofType0_Block {
    pub a: u8,
    pub b: u32,
    pub c: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ExprSizeofType0_Block {
    type Root = ExprSizeofType0;
    type ParentStack = (&'r ExprSizeofType0, <ExprSizeofType0 as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.a = _io.read_u1()?;
        self.b = _io.read_u4le()?;
        self.c = _io.read_bytes(2 as usize)?.to_vec();
        Ok(())
    }
}
impl<'r, 's: 'r> ExprSizeofType0_Block {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprSizeofType0_Block::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
