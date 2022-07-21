// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ExprBytesNonLiteral {
    pub one: u8,
    pub two: u8,
    pub calc_bytes: Option<Vec<u8>>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ExprBytesNonLiteral {
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
        Ok(())
    }
}
impl<'r, 's: 'r> ExprBytesNonLiteral {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprBytesNonLiteral::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn calc_bytes<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&Vec<u8>> {
        if self.calc_bytes.is_some() {
            return Ok(self.calc_bytes.as_ref().unwrap());
        }
        self.calc_bytes = Some(pack('C*', self.one, self.two) as Vec<u8>);
        return Ok(self.calc_bytes.as_ref().unwrap());
    }
}
