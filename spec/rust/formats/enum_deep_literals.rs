// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct EnumDeepLiterals {
    pub pet_1: Option<EnumDeepLiterals_Container1_Animal>,
    pub pet_2: Option<EnumDeepLiterals_Container1_Container2_Animal>,
    pub is_pet_1_ok: Option<bool>,
    pub is_pet_2_ok: Option<bool>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for EnumDeepLiterals {
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
impl<'r, 's: 'r> EnumDeepLiterals {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EnumDeepLiterals::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn is_pet_1_ok<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.is_pet_1_ok.is_some() {
            return Ok(self.is_pet_1_ok.as_ref().unwrap());
        }
        self.is_pet_1_ok = Some(self.pet_1 == EnumDeepLiterals_Container1_Animal::Cat as bool);
        return Ok(self.is_pet_1_ok.as_ref().unwrap());
    }
    fn is_pet_2_ok<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.is_pet_2_ok.is_some() {
            return Ok(self.is_pet_2_ok.as_ref().unwrap());
        }
        self.is_pet_2_ok = Some(self.pet_2 == EnumDeepLiterals_Container1_Container2_Animal::Hare as bool);
        return Ok(self.is_pet_2_ok.as_ref().unwrap());
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct EnumDeepLiterals_Container1 {
}
impl<'r, 's: 'r> KStruct<'r, 's> for EnumDeepLiterals_Container1 {
    type Root = EnumDeepLiterals;
    type ParentStack = (&'r EnumDeepLiterals, <EnumDeepLiterals as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        Ok(())
    }
}
impl<'r, 's: 'r> EnumDeepLiterals_Container1 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EnumDeepLiterals_Container1::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
#[derive(Debug, PartialEq)]
pub enum EnumDeepLiterals_Container1_Animal {
    Dog,
    Cat,
    Chicken,
}
impl TryFrom<i64> for EnumDeepLiterals_Container1_Animal {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<EnumDeepLiterals_Container1_Animal> {
        match flag {
            4 => Ok(EnumDeepLiterals_Container1_Animal::Dog),
            7 => Ok(EnumDeepLiterals_Container1_Animal::Cat),
            12 => Ok(EnumDeepLiterals_Container1_Animal::Chicken),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}


#[derive(Default, Debug, PartialEq)]
pub struct EnumDeepLiterals_Container1_Container2 {
}
impl<'r, 's: 'r> KStruct<'r, 's> for EnumDeepLiterals_Container1_Container2 {
    type Root = EnumDeepLiterals;
    type ParentStack = (&'r EnumDeepLiterals_Container1, <EnumDeepLiterals_Container1 as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        Ok(())
    }
}
impl<'r, 's: 'r> EnumDeepLiterals_Container1_Container2 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EnumDeepLiterals_Container1_Container2::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
#[derive(Debug, PartialEq)]
pub enum EnumDeepLiterals_Container1_Container2_Animal {
    Canary,
    Turtle,
    Hare,
}
impl TryFrom<i64> for EnumDeepLiterals_Container1_Container2_Animal {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<EnumDeepLiterals_Container1_Container2_Animal> {
        match flag {
            4 => Ok(EnumDeepLiterals_Container1_Container2_Animal::Canary),
            7 => Ok(EnumDeepLiterals_Container1_Container2_Animal::Turtle),
            12 => Ok(EnumDeepLiterals_Container1_Container2_Animal::Hare),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}

