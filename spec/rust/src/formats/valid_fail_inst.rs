// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ValidFailInst {
    pub a: u8,
    pub inst: Option<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ValidFailInst {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        {
            // condIfHeader(Compare(Name(identifier(inst)),GtE,IntNum(0)))
            self.a = _io.read_u1()?;
        }
        Ok(())
    }
}
impl<'r, 's: 'r> ValidFailInst {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ValidFailInst::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn inst<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u8> {
        if self.inst.is_some() {
            return Ok(self.inst.as_ref().unwrap());
        }
        // pushPos(_io)
        // seek(_io, IntNum(5))
        // popPos(_io)
        return Ok(self.inst.as_ref().unwrap());
    }
}
