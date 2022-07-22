// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct EnumNegative {
    pub f1: Option<EnumNegative_Constants>,
    pub f2: Option<EnumNegative_Constants>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for EnumNegative {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.f1 = Some((_io.read_s1()? as i64).try_into()?);
        self.f2 = Some((_io.read_s1()? as i64).try_into()?);
        Ok(())
    }
}
impl<'r, 's: 'r> EnumNegative {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EnumNegative::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
#[derive(Debug, PartialEq)]
pub enum EnumNegative_Constants {
    NegativeOne,
    PositiveOne,
}
impl TryFrom<i64> for EnumNegative_Constants {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<EnumNegative_Constants> {
        match flag {
            -1 => Ok(EnumNegative_Constants::NegativeOne),
            1 => Ok(EnumNegative_Constants::PositiveOne),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}

