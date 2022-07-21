// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
use crate::imports_circular_b::ImportsCircularB;

#[derive(Default, Debug, PartialEq)]
pub struct ImportsCircularA {
    pub code: u8,
    pub two: Option<Box<ImportsCircularB>>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ImportsCircularA {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.code = _io.read_u1()?;
        self.two = Some(Self::read_into::<BytesReader, Box<ImportsCircularB>>(Self::read_into::<S, ImportsCircularB>(_io)?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> ImportsCircularA {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ImportsCircularA::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
