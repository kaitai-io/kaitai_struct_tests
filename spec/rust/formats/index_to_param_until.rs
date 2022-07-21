// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct IndexToParamUntil {
    pub qty: u32,
    pub sizes: Vec<u32>,
    pub blocks: Vec<IndexToParamUntil_Block>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for IndexToParamUntil {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.qty = _io.read_u4le()?;
        self.sizes = Vec::new();
        {
            // condRepeatExprHeader(NamedIdentifier(sizes), _io, IntMultiType(false,Width4,Some(LittleEndian)), Name(identifier(qty)))
            // handleAssignmentRepeatExpr(NamedIdentifier(sizes), _io.read_u4le()?)
        }
        self.blocks = Vec::new();
        {
            // condRepeatUntilHeader(NamedIdentifier(blocks), _io, UserTypeInstream(List(block),None,ArrayBuffer(Name(identifier(_index)))), Attribute(Name(identifier(_io)),identifier(eof)))
            // handleAssignmentRepeatUntil(NamedIdentifier(blocks), Self::read_into::<S, IndexToParamUntil_Block>(i, _io, _root, _parent.push(self))?.into(), false)
            // condRepeatUntilFooter(NamedIdentifier(blocks), _io, UserTypeInstream(List(block),None,ArrayBuffer(Name(identifier(_index)))), Attribute(Name(identifier(_io)),identifier(eof)))
        } {}
        Ok(())
    }
}
impl<'r, 's: 'r> IndexToParamUntil {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = IndexToParamUntil::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct IndexToParamUntil_Block {
    pub idx: i32,
    pub buf: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for IndexToParamUntil_Block {
    type Root = IndexToParamUntil;
    type ParentStack = (&'r IndexToParamUntil, <IndexToParamUntil as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.buf = decode_string(_io.read_bytes(_root.ok_or(KError::MissingRoot)?.sizes[self.idx as usize] as usize)?, "ASCII")?;
        Ok(())
    }
}
impl<'r, 's: 'r> IndexToParamUntil_Block {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = IndexToParamUntil_Block::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
