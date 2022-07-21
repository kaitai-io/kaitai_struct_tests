// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct EnumIntRangeS {
    pub f1: Option<EnumIntRangeS_Constants>,
    pub f2: Option<EnumIntRangeS_Constants>,
    pub f3: Option<EnumIntRangeS_Constants>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for EnumIntRangeS {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.f1 = Some((_io.read_s4be()? as i64).try_into()?);
        self.f2 = Some((_io.read_s4be()? as i64).try_into()?);
        self.f3 = Some((_io.read_s4be()? as i64).try_into()?);
        Ok(())
    }
}
impl<'r, 's: 'r> EnumIntRangeS {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EnumIntRangeS::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
#[derive(Debug, PartialEq)]
pub enum EnumIntRangeS_Constants {
    IntMin,
    Zero,
    IntMax,
}
impl TryFrom<i64> for EnumIntRangeS_Constants {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<EnumIntRangeS_Constants> {
        match flag {
            -2147483648 => Ok(EnumIntRangeS_Constants::IntMin),
            0 => Ok(EnumIntRangeS_Constants::Zero),
            2147483647 => Ok(EnumIntRangeS_Constants::IntMax),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}

