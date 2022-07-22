// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ExprEnum {
    pub one: u8,
    pub const_dog: Option<ExprEnum_Animal>,
    pub derived_boom: Option<ExprEnum_Animal>,
    pub derived_dog: Option<ExprEnum_Animal>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ExprEnum {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.one = _io.read_u1()?;
        Ok(())
    }
}
impl<'r, 's: 'r> ExprEnum {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprEnum::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn const_dog<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&ExprEnum_Animal> {
        if self.const_dog.is_some() {
            return Ok(self.const_dog.as_ref().unwrap());
        }
        self.const_dog = Some(4 as i32);
        return Ok(self.const_dog.as_ref().unwrap());
    }
    fn derived_boom<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&ExprEnum_Animal> {
        if self.derived_boom.is_some() {
            return Ok(self.derived_boom.as_ref().unwrap());
        }
        self.derived_boom = Some(self.one as i32);
        return Ok(self.derived_boom.as_ref().unwrap());
    }
    fn derived_dog<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&ExprEnum_Animal> {
        if self.derived_dog.is_some() {
            return Ok(self.derived_dog.as_ref().unwrap());
        }
        self.derived_dog = Some((self.one - 98) as i32);
        return Ok(self.derived_dog.as_ref().unwrap());
    }
}
#[derive(Debug, PartialEq)]
pub enum ExprEnum_Animal {
    Dog,
    Cat,
    Chicken,
    Boom,
}
impl TryFrom<i64> for ExprEnum_Animal {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<ExprEnum_Animal> {
        match flag {
            4 => Ok(ExprEnum_Animal::Dog),
            7 => Ok(ExprEnum_Animal::Cat),
            12 => Ok(ExprEnum_Animal::Chicken),
            102 => Ok(ExprEnum_Animal::Boom),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}

