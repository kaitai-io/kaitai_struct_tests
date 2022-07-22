// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
// extraAttrForIO(RawIdentifier(NamedIdentifier(block1)), NoRepeat)

#[derive(Default, Debug, PartialEq)]
pub struct ExprSizeofValueSized {
    pub block1: Option<ExprSizeofValueSized_Block>,
    pub more: u16,
    pub raw_block1: Vec<u8>,
    pub self_sizeof: Option<i32>,
    pub sizeof_block: Option<i32>,
    pub sizeof_block_b: Option<i32>,
    pub sizeof_block_a: Option<i32>,
    pub sizeof_block_c: Option<i32>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ExprSizeofValueSized {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.block1 = Some(Self::read_into::<BytesReader, ExprSizeofValueSized_Block>(&BytesReader::new(_io.read_bytes(12 as usize)?), Some(self), _parent.push(self))?);
        self.more = _io.read_u2le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> ExprSizeofValueSized {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprSizeofValueSized::default();

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
        self.self_sizeof = Some(14 as i32);
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
        self.sizeof_block = Some(12 as i32);
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
pub struct ExprSizeofValueSized_Block {
    pub a: u8,
    pub b: u32,
    pub c: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ExprSizeofValueSized_Block {
    type Root = ExprSizeofValueSized;
    type ParentStack = (&'r ExprSizeofValueSized, <ExprSizeofValueSized as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> ExprSizeofValueSized_Block {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprSizeofValueSized_Block::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
