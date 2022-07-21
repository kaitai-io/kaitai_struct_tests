// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct PositionAbs {
    pub index_offset: u32,
    pub index: Option<PositionAbs_IndexObj>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for PositionAbs {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.index_offset = _io.read_u4le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> PositionAbs {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = PositionAbs::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn index<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&PositionAbs_IndexObj> {
        if self.index.is_some() {
            return Ok(self.index.as_ref().unwrap());
        }
        // pushPos(_io)
        // seek(_io, Name(identifier(index_offset)))
        // popPos(_io)
        return Ok(self.index.as_ref().unwrap());
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct PositionAbs_IndexObj {
    pub entry: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for PositionAbs_IndexObj {
    type Root = PositionAbs;
    type ParentStack = (&'r PositionAbs, <PositionAbs as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.entry = decode_string(_io.read_bytes_term(0, false, true, true)?, "UTF-8")?;
        Ok(())
    }
}
impl<'r, 's: 'r> PositionAbs_IndexObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = PositionAbs_IndexObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
