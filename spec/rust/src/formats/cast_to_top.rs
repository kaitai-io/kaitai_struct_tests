// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct CastToTop {
    pub code: u8,
    pub header: Option<Box<CastToTop>>,
    pub header_casted: Option<Box<CastToTop>>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for CastToTop {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.code = _io.read_u1()?;
        Ok(())
    }
}
impl<'r, 's: 'r> CastToTop {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = CastToTop::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn header<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&Box<CastToTop>> {
        if self.header.is_some() {
            return Ok(self.header.as_ref().unwrap());
        }
        // pushPos(_io)
        // seek(_io, IntNum(1))
        // popPos(_io)
        return Ok(self.header.as_ref().unwrap());
    }
    fn header_casted<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&Box<CastToTop>> {
        if self.header_casted.is_some() {
            return Ok(self.header_casted.as_ref().unwrap());
        }
        self.header_casted = Some(self.header(_io, _root, _parent)? as CastToTop);
        return Ok(self.header_casted.as_ref().unwrap());
    }
}
