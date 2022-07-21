// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
use crate::hello_world::HelloWorld;

#[derive(Default, Debug, PartialEq)]
pub struct CastToImported {
    pub one: Option<Box<HelloWorld>>,
    pub one_casted: Option<Box<HelloWorld>>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for CastToImported {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.one = Some(Self::read_into::<BytesReader, Box<HelloWorld>>(Self::read_into::<S, HelloWorld>(_io)?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> CastToImported {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = CastToImported::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn one_casted<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&Box<HelloWorld>> {
        if self.one_casted.is_some() {
            return Ok(self.one_casted.as_ref().unwrap());
        }
        self.one_casted = Some(self.one as HelloWorld);
        return Ok(self.one_casted.as_ref().unwrap());
    }
}
