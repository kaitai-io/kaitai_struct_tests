// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ParamsEnum {
    pub one: Option<ParamsEnum_Animal>,
    pub invoke_with_param: Option<ParamsEnum_WithParam>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ParamsEnum {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.one = Some((_io.read_u1()? as i64).try_into()?);
        self.invoke_with_param = Some(Self::read_into::<BytesReader, ParamsEnum_WithParam>(Self::read_into::<S, ParamsEnum_WithParam>(self.one, _io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> ParamsEnum {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ParamsEnum::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
#[derive(Debug, PartialEq)]
pub enum ParamsEnum_Animal {
    Dog,
    Cat,
    Chicken,
}
impl TryFrom<i64> for ParamsEnum_Animal {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<ParamsEnum_Animal> {
        match flag {
            4 => Ok(ParamsEnum_Animal::Dog),
            7 => Ok(ParamsEnum_Animal::Cat),
            12 => Ok(ParamsEnum_Animal::Chicken),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}


#[derive(Default, Debug, PartialEq)]
pub struct ParamsEnum_WithParam {
    pub enumerated_one: Option<ParamsEnum_Animal>,
    pub is_cat: Option<bool>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ParamsEnum_WithParam {
    type Root = ParamsEnum;
    type ParentStack = (&'r ParamsEnum, <ParamsEnum as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        Ok(())
    }
}
impl<'r, 's: 'r> ParamsEnum_WithParam {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ParamsEnum_WithParam::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn is_cat<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r ParamsEnum>,
        _parent: Option<TypedStack<(&'r ParamsEnum, <ParamsEnum as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&bool> {
        if self.is_cat.is_some() {
            return Ok(self.is_cat.as_ref().unwrap());
        }
        self.is_cat = Some(self.enumerated_one == ParamsEnum_Animal::Cat as bool);
        return Ok(self.is_cat.as_ref().unwrap());
    }
}
