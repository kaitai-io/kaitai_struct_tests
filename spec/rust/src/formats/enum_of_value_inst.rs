// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct EnumOfValueInst {
    pub pet_1: Option<EnumOfValueInst_Animal>,
    pub pet_2: Option<EnumOfValueInst_Animal>,
    pub pet_3: Option<EnumOfValueInst_Animal>,
    pub pet_4: Option<EnumOfValueInst_Animal>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for EnumOfValueInst {
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
impl<'r, 's: 'r> EnumOfValueInst {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EnumOfValueInst::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn pet_3<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&EnumOfValueInst_Animal> {
        if self.pet_3.is_some() {
            return Ok(self.pet_3.as_ref().unwrap());
        }
        self.pet_3 = Some(if self.pet_1 == EnumOfValueInst_Animal::Cat { 4 } else { 12} as i32);
        return Ok(self.pet_3.as_ref().unwrap());
    }
    fn pet_4<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&EnumOfValueInst_Animal> {
        if self.pet_4.is_some() {
            return Ok(self.pet_4.as_ref().unwrap());
        }
        self.pet_4 = Some(if self.pet_1 == EnumOfValueInst_Animal::Cat { EnumOfValueInst_Animal::Dog } else { EnumOfValueInst_Animal::Chicken} as i32);
        return Ok(self.pet_4.as_ref().unwrap());
    }
}
#[derive(Debug, PartialEq)]
pub enum EnumOfValueInst_Animal {
    Dog,
    Cat,
    Chicken,
}
impl TryFrom<i64> for EnumOfValueInst_Animal {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<EnumOfValueInst_Animal> {
        match flag {
            4 => Ok(EnumOfValueInst_Animal::Dog),
            7 => Ok(EnumOfValueInst_Animal::Cat),
            12 => Ok(EnumOfValueInst_Animal::Chicken),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}

