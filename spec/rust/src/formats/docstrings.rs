// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
// universalDoc()

#[derive(Default, Debug, PartialEq)]
pub struct Docstrings {
    pub one: u8,
    pub two: Option<u8>,
    pub three: Option<i8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for Docstrings {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.one = _io.read_u1()?;
        Ok(())
    }
}
impl<'r, 's: 'r> Docstrings {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = Docstrings::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    // universalDoc()
    fn two<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u8> {
        if self.two.is_some() {
            return Ok(self.two.as_ref().unwrap());
        }
        // pushPos(_io)
        // seek(_io, IntNum(0))
        // popPos(_io)
        return Ok(self.two.as_ref().unwrap());
    }
    // universalDoc()
    fn three<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i8> {
        if self.three.is_some() {
            return Ok(self.three.as_ref().unwrap());
        }
        self.three = Some(66 as i8);
        return Ok(self.three.as_ref().unwrap());
    }
}
// universalDoc()
// universalDoc()

#[derive(Default, Debug, PartialEq)]
pub struct Docstrings_ComplexSubtype {
}
impl<'r, 's: 'r> KStruct<'r, 's> for Docstrings_ComplexSubtype {
    type Root = Docstrings;
    type ParentStack = (&'r Docstrings, <Docstrings as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        Ok(())
    }
}
impl<'r, 's: 'r> Docstrings_ComplexSubtype {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = Docstrings_ComplexSubtype::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
