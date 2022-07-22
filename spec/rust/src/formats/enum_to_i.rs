// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct EnumToI {
    pub pet_1: Option<EnumToI_Animal>,
    pub pet_2: Option<EnumToI_Animal>,
    pub pet_1_i: Option<i32>,
    pub pet_1_eq_int: Option<bool>,
    pub one_lt_two: Option<bool>,
    pub pet_1_mod: Option<i32>,
    pub pet_2_eq_int: Option<bool>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for EnumToI {
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
impl<'r, 's: 'r> EnumToI {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EnumToI::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn pet_1_i<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.pet_1_i.is_some() {
            return Ok(self.pet_1_i.as_ref().unwrap());
        }
        self.pet_1_i = Some(self.pet_1 as i32);
        return Ok(self.pet_1_i.as_ref().unwrap());
    }
    fn pet_1_eq_int<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.pet_1_eq_int.is_some() {
            return Ok(self.pet_1_eq_int.as_ref().unwrap());
        }
        self.pet_1_eq_int = Some(self.pet_1 == 7 as bool);
        return Ok(self.pet_1_eq_int.as_ref().unwrap());
    }
    fn one_lt_two<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.one_lt_two.is_some() {
            return Ok(self.one_lt_two.as_ref().unwrap());
        }
        self.one_lt_two = Some(self.pet_1 < self.pet_2 as bool);
        return Ok(self.one_lt_two.as_ref().unwrap());
    }
    fn pet_1_mod<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.pet_1_mod.is_some() {
            return Ok(self.pet_1_mod.as_ref().unwrap());
        }
        self.pet_1_mod = Some((self.pet_1 + 32768) as i32);
        return Ok(self.pet_1_mod.as_ref().unwrap());
    }
    fn pet_2_eq_int<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.pet_2_eq_int.is_some() {
            return Ok(self.pet_2_eq_int.as_ref().unwrap());
        }
        self.pet_2_eq_int = Some(self.pet_2 == 5 as bool);
        return Ok(self.pet_2_eq_int.as_ref().unwrap());
    }
}
#[derive(Debug, PartialEq)]
pub enum EnumToI_Animal {
    Dog,
    Cat,
    Chicken,
}
impl TryFrom<i64> for EnumToI_Animal {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<EnumToI_Animal> {
        match flag {
            4 => Ok(EnumToI_Animal::Dog),
            7 => Ok(EnumToI_Animal::Cat),
            12 => Ok(EnumToI_Animal::Chicken),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}

