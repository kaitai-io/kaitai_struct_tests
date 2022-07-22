// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct BitsEnum {
    pub one: Option<BitsEnum_Animal>,
    pub two: Option<BitsEnum_Animal>,
    pub three: Option<BitsEnum_Animal>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for BitsEnum {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.one = Some((_io.read_bits_int(4)? as i64).try_into()?);
        self.two = Some((_io.read_bits_int(8)? as i64).try_into()?);
        self.three = Some((_io.read_bits_int(1)? as i64).try_into()?);
        Ok(())
    }
}
impl<'r, 's: 'r> BitsEnum {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = BitsEnum::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
#[derive(Debug, PartialEq)]
pub enum BitsEnum_Animal {
    Cat,
    Dog,
    Horse,
    Platypus,
}
impl TryFrom<i64> for BitsEnum_Animal {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<BitsEnum_Animal> {
        match flag {
            0 => Ok(BitsEnum_Animal::Cat),
            1 => Ok(BitsEnum_Animal::Dog),
            4 => Ok(BitsEnum_Animal::Horse),
            5 => Ok(BitsEnum_Animal::Platypus),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}

