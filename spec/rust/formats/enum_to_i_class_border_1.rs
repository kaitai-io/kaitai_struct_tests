// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
use crate::enum_to_i_class_border_2::EnumToIClassBorder2;

#[derive(Default, Debug, PartialEq)]
pub struct EnumToIClassBorder1 {
    pub pet_1: Option<EnumToIClassBorder1_Animal>,
    pub pet_2: Option<EnumToIClassBorder1_Animal>,
    pub some_dog: Option<EnumToIClassBorder1_Animal>,
    pub checker: Option<Box<EnumToIClassBorder2>>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for EnumToIClassBorder1 {
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
impl<'r, 's: 'r> EnumToIClassBorder1 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EnumToIClassBorder1::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn some_dog<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&EnumToIClassBorder1_Animal> {
        if self.some_dog.is_some() {
            return Ok(self.some_dog.as_ref().unwrap());
        }
        self.some_dog = Some(4 as i32);
        return Ok(self.some_dog.as_ref().unwrap());
    }
    fn checker<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&Box<EnumToIClassBorder2>> {
        if self.checker.is_some() {
            return Ok(self.checker.as_ref().unwrap());
        }
        // pushPos(_io)
        // seek(_io, IntNum(0))
        // popPos(_io)
        return Ok(self.checker.as_ref().unwrap());
    }
}
#[derive(Debug, PartialEq)]
pub enum EnumToIClassBorder1_Animal {
    Dog,
    Cat,
    Chicken,
}
impl TryFrom<i64> for EnumToIClassBorder1_Animal {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<EnumToIClassBorder1_Animal> {
        match flag {
            4 => Ok(EnumToIClassBorder1_Animal::Dog),
            7 => Ok(EnumToIClassBorder1_Animal::Cat),
            12 => Ok(EnumToIClassBorder1_Animal::Chicken),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}

