// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct CombineEnum {
    pub enum_u4: Option<CombineEnum_Animal>,
    pub enum_u2: Option<CombineEnum_Animal>,
    pub enum_u4_u2: Option<CombineEnum_Animal>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for CombineEnum {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.enum_u4 = Some((_io.read_u4le()? as i64).try_into()?);
        self.enum_u2 = Some((_io.read_u2le()? as i64).try_into()?);
        Ok(())
    }
}
impl<'r, 's: 'r> CombineEnum {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = CombineEnum::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn enum_u4_u2<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&CombineEnum_Animal> {
        if self.enum_u4_u2.is_some() {
            return Ok(self.enum_u4_u2.as_ref().unwrap());
        }
        self.enum_u4_u2 = Some(if false { self.enum_u4 } else { self.enum_u2} as i32);
        return Ok(self.enum_u4_u2.as_ref().unwrap());
    }
}
#[derive(Debug, PartialEq)]
pub enum CombineEnum_Animal {
    Pig,
    Horse,
}
impl TryFrom<i64> for CombineEnum_Animal {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<CombineEnum_Animal> {
        match flag {
            7 => Ok(CombineEnum_Animal::Pig),
            12 => Ok(CombineEnum_Animal::Horse),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}

