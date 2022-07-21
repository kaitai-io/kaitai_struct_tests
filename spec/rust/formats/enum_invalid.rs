// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct EnumInvalid {
    pub pet_1: Option<EnumInvalid_Animal>,
    pub pet_2: Option<EnumInvalid_Animal>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for EnumInvalid {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.pet_1 = Some((_io.read_u1()? as i64).try_into()?);
        self.pet_2 = Some((_io.read_u1()? as i64).try_into()?);
        Ok(())
    }
}
impl<'r, 's: 'r> EnumInvalid {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EnumInvalid::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
#[derive(Debug, PartialEq)]
pub enum EnumInvalid_Animal {
    Dog,
    Cat,
}
impl TryFrom<i64> for EnumInvalid_Animal {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<EnumInvalid_Animal> {
        match flag {
            102 => Ok(EnumInvalid_Animal::Dog),
            124 => Ok(EnumInvalid_Animal::Cat),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}

