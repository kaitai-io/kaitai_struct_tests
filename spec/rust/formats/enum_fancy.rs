// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct EnumFancy {
    pub pet_1: Option<EnumFancy_Animal>,
    pub pet_2: Option<EnumFancy_Animal>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for EnumFancy {
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
impl<'r, 's: 'r> EnumFancy {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EnumFancy::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
#[derive(Debug, PartialEq)]
pub enum EnumFancy_Animal {
    // universalDoc()
    Dog,
    // universalDoc()
    Cat,
    Chicken,
}
impl TryFrom<i64> for EnumFancy_Animal {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<EnumFancy_Animal> {
        match flag {
            4 => Ok(EnumFancy_Animal::Dog),
            7 => Ok(EnumFancy_Animal::Cat),
            12 => Ok(EnumFancy_Animal::Chicken),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}

