// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct EnumDeep {
    pub pet_1: Option<EnumDeep_Container1_Animal>,
    pub pet_2: Option<EnumDeep_Container1_Container2_Animal>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for EnumDeep {
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
impl<'r, 's: 'r> EnumDeep {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EnumDeep::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct EnumDeep_Container1 {
}
impl<'r, 's: 'r> KStruct<'r, 's> for EnumDeep_Container1 {
    type Root = EnumDeep;
    type ParentStack = (&'r EnumDeep, <EnumDeep as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        Ok(())
    }
}
impl<'r, 's: 'r> EnumDeep_Container1 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EnumDeep_Container1::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
#[derive(Debug, PartialEq)]
pub enum EnumDeep_Container1_Animal {
    Dog,
    Cat,
    Chicken,
}
impl TryFrom<i64> for EnumDeep_Container1_Animal {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<EnumDeep_Container1_Animal> {
        match flag {
            4 => Ok(EnumDeep_Container1_Animal::Dog),
            7 => Ok(EnumDeep_Container1_Animal::Cat),
            12 => Ok(EnumDeep_Container1_Animal::Chicken),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}


#[derive(Default, Debug, PartialEq)]
pub struct EnumDeep_Container1_Container2 {
}
impl<'r, 's: 'r> KStruct<'r, 's> for EnumDeep_Container1_Container2 {
    type Root = EnumDeep;
    type ParentStack = (&'r EnumDeep_Container1, <EnumDeep_Container1 as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        Ok(())
    }
}
impl<'r, 's: 'r> EnumDeep_Container1_Container2 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EnumDeep_Container1_Container2::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
#[derive(Debug, PartialEq)]
pub enum EnumDeep_Container1_Container2_Animal {
    Canary,
    Turtle,
    Hare,
}
impl TryFrom<i64> for EnumDeep_Container1_Container2_Animal {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<EnumDeep_Container1_Container2_Animal> {
        match flag {
            4 => Ok(EnumDeep_Container1_Container2_Animal::Canary),
            7 => Ok(EnumDeep_Container1_Container2_Animal::Turtle),
            12 => Ok(EnumDeep_Container1_Container2_Animal::Hare),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}

