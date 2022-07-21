// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct EnumForUnknownId {
    pub one: Option<EnumForUnknownId_Animal>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for EnumForUnknownId {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.one = Some((_io.read_u1()? as i64).try_into()?);
        Ok(())
    }
}
impl<'r, 's: 'r> EnumForUnknownId {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EnumForUnknownId::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
#[derive(Debug, PartialEq)]
pub enum EnumForUnknownId_Animal {
    Dog,
    Cat,
    Chicken,
}
impl TryFrom<i64> for EnumForUnknownId_Animal {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<EnumForUnknownId_Animal> {
        match flag {
            4 => Ok(EnumForUnknownId_Animal::Dog),
            7 => Ok(EnumForUnknownId_Animal::Cat),
            12 => Ok(EnumForUnknownId_Animal::Chicken),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}

