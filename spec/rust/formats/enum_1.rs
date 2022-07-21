// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct Enum1 {
    pub main: Option<Enum1_MainObj>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for Enum1 {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.main = Some(Self::read_into::<BytesReader, Enum1_MainObj>(Self::read_into::<S, Enum1_MainObj>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> Enum1 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = Enum1::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct Enum1_MainObj {
    pub submain: Option<Enum1_MainObj_SubmainObj>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for Enum1_MainObj {
    type Root = Enum1;
    type ParentStack = (&'r Enum1, <Enum1 as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.submain = Some(Self::read_into::<BytesReader, Enum1_MainObj_SubmainObj>(Self::read_into::<S, Enum1_MainObj_SubmainObj>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> Enum1_MainObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = Enum1_MainObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
#[derive(Debug, PartialEq)]
pub enum Enum1_MainObj_Animal {
    Dog,
    Cat,
    Chicken,
}
impl TryFrom<i64> for Enum1_MainObj_Animal {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<Enum1_MainObj_Animal> {
        match flag {
            4 => Ok(Enum1_MainObj_Animal::Dog),
            7 => Ok(Enum1_MainObj_Animal::Cat),
            12 => Ok(Enum1_MainObj_Animal::Chicken),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}


#[derive(Default, Debug, PartialEq)]
pub struct Enum1_MainObj_SubmainObj {
    pub pet_1: Option<Enum1_MainObj_Animal>,
    pub pet_2: Option<Enum1_MainObj_Animal>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for Enum1_MainObj_SubmainObj {
    type Root = Enum1;
    type ParentStack = (&'r Enum1_MainObj, <Enum1_MainObj as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> Enum1_MainObj_SubmainObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = Enum1_MainObj_SubmainObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
