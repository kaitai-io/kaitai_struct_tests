// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ExprSizeofValue0 {
    pub block1: Option<ExprSizeofValue0_Block>,
    pub more: u16,
    pub self_sizeof: Option<i32>,
    pub sizeof_block: Option<i32>,
    pub sizeof_block_b: Option<i32>,
    pub sizeof_block_a: Option<i32>,
    pub sizeof_block_c: Option<i32>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ExprSizeofValue0 {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.block1 = Some(Self::read_into::<BytesReader, ExprSizeofValue0_Block>(Self::read_into::<S, ExprSizeofValue0_Block>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.more = _io.read_u2le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> ExprSizeofValue0 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprSizeofValue0::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn self_sizeof<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.self_sizeof.is_some() {
            return Ok(self.self_sizeof.as_ref().unwrap());
        }
        self.self_sizeof = Some(9 as i32);
        return Ok(self.self_sizeof.as_ref().unwrap());
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
    fn sizeof_block_b<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.sizeof_block_b.is_some() {
            return Ok(self.sizeof_block_b.as_ref().unwrap());
        }
        self.sizeof_block_b = Some(4 as i32);
        return Ok(self.sizeof_block_b.as_ref().unwrap());
    }
    fn sizeof_block_a<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.sizeof_block_a.is_some() {
            return Ok(self.sizeof_block_a.as_ref().unwrap());
        }
        self.sizeof_block_a = Some(1 as i32);
        return Ok(self.sizeof_block_a.as_ref().unwrap());
    }
    fn sizeof_block_c<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.sizeof_block_c.is_some() {
            return Ok(self.sizeof_block_c.as_ref().unwrap());
        }
        self.sizeof_block_c = Some(2 as i32);
        return Ok(self.sizeof_block_c.as_ref().unwrap());
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct ExprSizeofValue0_Block {
    pub a: u8,
    pub b: u32,
    pub c: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ExprSizeofValue0_Block {
    type Root = ExprSizeofValue0;
    type ParentStack = (&'r ExprSizeofValue0, <ExprSizeofValue0 as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> ExprSizeofValue0_Block {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprSizeofValue0_Block::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
