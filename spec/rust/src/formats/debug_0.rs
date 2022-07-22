// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct Debug0 {
    pub one: u8,
    pub array_of_ints: Vec<u8>,
    pub _unnamed2: u8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for Debug0 {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.one = _io.read_u1()?;
        self.array_of_ints = Vec::new();
        {
            // condRepeatExprHeader(NamedIdentifier(array_of_ints), _io, Int1Type(false), IntNum(3))
            // handleAssignmentRepeatExpr(NamedIdentifier(array_of_ints), _io.read_u1()?)
        }
        self._unnamed2 = _io.read_u1()?;
        Ok(())
    }
}
impl<'r, 's: 'r> Debug0 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = Debug0::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
