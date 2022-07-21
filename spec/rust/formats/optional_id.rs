// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct OptionalId {
    pub _unnamed0: u8,
    pub _unnamed1: u8,
    pub _unnamed2: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for OptionalId {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self._unnamed0 = _io.read_u1()?;
        self._unnamed1 = _io.read_u1()?;
        self._unnamed2 = _io.read_bytes(5 as usize)?.to_vec();
        Ok(())
    }
}
impl<'r, 's: 'r> OptionalId {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = OptionalId::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
