// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
use crate::hello_world::HelloWorld;

#[derive(Default, Debug, PartialEq)]
pub struct Imports0 {
    pub two: u8,
    pub hw: Option<Box<HelloWorld>>,
    pub hw_one: Option<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for Imports0 {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.two = _io.read_u1()?;
        self.hw = Some(Self::read_into::<BytesReader, Box<HelloWorld>>(Self::read_into::<S, HelloWorld>(_io)?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> Imports0 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = Imports0::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn hw_one<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u8> {
        if self.hw_one.is_some() {
            return Ok(self.hw_one.as_ref().unwrap());
        }
        self.hw_one = Some(self.hw.one as u8);
        return Ok(self.hw_one.as_ref().unwrap());
    }
}
