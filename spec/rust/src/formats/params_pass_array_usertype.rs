// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ParamsPassArrayUsertype {
    pub blocks: Vec<ParamsPassArrayUsertype_Block>,
    pub pass_blocks: Option<ParamsPassArrayUsertype_ParamType>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ParamsPassArrayUsertype {
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
            // condRepeatExprHeader(NamedIdentifier(blocks), _io, UserTypeInstream(List(block),None,List()), IntNum(2))
            // handleAssignmentRepeatExpr(NamedIdentifier(blocks), Self::read_into::<S, ParamsPassArrayUsertype_Block>(_io, _root, _parent.push(self))?.into())
        }
        self.pass_blocks = Some(Self::read_into::<BytesReader, ParamsPassArrayUsertype_ParamType>(Self::read_into::<S, ParamsPassArrayUsertype_ParamType>(self.blocks, _io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> ParamsPassArrayUsertype {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ParamsPassArrayUsertype::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct ParamsPassArrayUsertype_Block {
    pub foo: u8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ParamsPassArrayUsertype_Block {
    type Root = ParamsPassArrayUsertype;
    type ParentStack = (&'r ParamsPassArrayUsertype, <ParamsPassArrayUsertype as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.foo = _io.read_u1()?;
        Ok(())
    }
}
impl<'r, 's: 'r> ParamsPassArrayUsertype_Block {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ParamsPassArrayUsertype_Block::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct ParamsPassArrayUsertype_ParamType {
    pub bar: Vec<ParamsPassArrayUsertype_Block>,
    pub one: Vec<u8>,
    pub two: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ParamsPassArrayUsertype_ParamType {
    type Root = ParamsPassArrayUsertype;
    type ParentStack = (&'r ParamsPassArrayUsertype, <ParamsPassArrayUsertype as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.one = _io.read_bytes(self.bar[0 as usize].foo as usize)?.to_vec();
        self.two = _io.read_bytes(self.bar[1 as usize].foo as usize)?.to_vec();
        Ok(())
    }
}
impl<'r, 's: 'r> ParamsPassArrayUsertype_ParamType {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ParamsPassArrayUsertype_ParamType::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
