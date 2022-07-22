// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ExprIfIntOps {
    pub skip: Vec<u8>,
    pub it: i16,
    pub boxed: i16,
    pub is_eq_prim: Option<bool>,
    pub is_eq_boxed: Option<bool>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ExprIfIntOps {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.skip = _io.read_bytes(2 as usize)?.to_vec();
        {
            // condIfHeader(Bool(true))
            self.it = _io.read_s2le()?;
        }
        {
            // condIfHeader(Bool(true))
            self.boxed = _io.read_s2le()?;
        }
        Ok(())
    }
}
impl<'r, 's: 'r> ExprIfIntOps {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprIfIntOps::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn is_eq_prim<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.is_eq_prim.is_some() {
            return Ok(self.is_eq_prim.as_ref().unwrap());
        }
        self.is_eq_prim = Some(self.it == 16705 as bool);
        return Ok(self.is_eq_prim.as_ref().unwrap());
    }
    fn is_eq_boxed<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.is_eq_boxed.is_some() {
            return Ok(self.is_eq_boxed.as_ref().unwrap());
        }
        self.is_eq_boxed = Some(self.it == self.boxed as bool);
        return Ok(self.is_eq_boxed.as_ref().unwrap());
    }
}
