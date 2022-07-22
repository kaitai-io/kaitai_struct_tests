// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
use crate::imports_circular_a::ImportsCircularA;

#[derive(Default, Debug, PartialEq)]
pub struct ImportsCircularB {
    pub initial: u8,
    pub back_ref: Option<Box<ImportsCircularA>>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ImportsCircularB {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.initial = _io.read_u1()?;
        {
            // condIfHeader(Compare(Name(identifier(initial)),Eq,IntNum(65)))
            self.back_ref = Some(Self::read_into::<BytesReader, Box<ImportsCircularA>>(Self::read_into::<S, ImportsCircularA>(_io)?.into(), Some(self), _parent.push(self))?);
        }
        Ok(())
    }
}
impl<'r, 's: 'r> ImportsCircularB {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ImportsCircularB::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
