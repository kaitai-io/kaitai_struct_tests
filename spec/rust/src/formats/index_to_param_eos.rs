// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct IndexToParamEos {
    pub qty: u32,
    pub sizes: Vec<u32>,
    pub blocks: Vec<IndexToParamEos_Block>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for IndexToParamEos {
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
            type ArrayElement = IndexToParamEos_Block;
            while !_io.is_eof() {
                self.blocks.push(Self::read_into::<S, ArrayElement>(_io, _root, _parent.push(self))?);
            }
        }
        Ok(())
    }
}
impl<'r, 's: 'r> IndexToParamEos {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = IndexToParamEos::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct IndexToParamEos_Block {
    pub idx: i32,
    pub buf: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for IndexToParamEos_Block {
    type Root = IndexToParamEos;
    type ParentStack = (&'r IndexToParamEos, <IndexToParamEos as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> IndexToParamEos_Block {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = IndexToParamEos_Block::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
