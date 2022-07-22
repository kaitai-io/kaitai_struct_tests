// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct Enum0 {
    pub pet_1: Option<Enum0_Animal>,
    pub pet_2: Option<Enum0_Animal>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for Enum0 {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.pet_1 = Some((_io.read_u4le()? as i64).try_into()?);
        self.pet_2 = Some((_io.read_u4le()? as i64).try_into()?);
        Ok(())
    }
}
impl<'r, 's: 'r> Enum0 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = Enum0::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
#[derive(Debug, PartialEq)]
pub enum Enum0_Animal {
    Dog,
    Cat,
    Chicken,
}
impl TryFrom<i64> for Enum0_Animal {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<Enum0_Animal> {
        match flag {
            4 => Ok(Enum0_Animal::Dog),
            7 => Ok(Enum0_Animal::Cat),
            12 => Ok(Enum0_Animal::Chicken),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}

