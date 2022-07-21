// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
// universalDoc()

#[derive(Default, Debug, PartialEq)]
pub struct DocstringsDocref {
    pub one: u8,
    pub two: u8,
    pub three: u8,
    pub foo: Option<bool>,
    pub parse_inst: Option<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for DocstringsDocref {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.one = _io.read_u1()?;
        self.two = _io.read_u1()?;
        self.three = _io.read_u1()?;
        Ok(())
    }
}
impl<'r, 's: 'r> DocstringsDocref {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = DocstringsDocref::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    // universalDoc()
    fn foo<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.foo.is_some() {
            return Ok(self.foo.as_ref().unwrap());
        }
        self.foo = Some(true as bool);
        return Ok(self.foo.as_ref().unwrap());
    }
    // universalDoc()
    fn parse_inst<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u8> {
        if self.parse_inst.is_some() {
            return Ok(self.parse_inst.as_ref().unwrap());
        }
        // pushPos(_io)
        // seek(_io, IntNum(0))
        // popPos(_io)
        return Ok(self.parse_inst.as_ref().unwrap());
    }
}
// universalDoc()
// universalDoc()
// universalDoc()
