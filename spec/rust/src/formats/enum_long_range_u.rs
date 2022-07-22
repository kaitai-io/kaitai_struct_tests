// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct EnumLongRangeU {
    pub f1: Option<EnumLongRangeU_Constants>,
    pub f2: Option<EnumLongRangeU_Constants>,
    pub f3: Option<EnumLongRangeU_Constants>,
    pub f4: Option<EnumLongRangeU_Constants>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for EnumLongRangeU {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.f1 = Some((_io.read_u8be()? as i64).try_into()?);
        self.f2 = Some((_io.read_u8be()? as i64).try_into()?);
        self.f3 = Some((_io.read_u8be()? as i64).try_into()?);
        self.f4 = Some((_io.read_u8be()? as i64).try_into()?);
        Ok(())
    }
}
impl<'r, 's: 'r> EnumLongRangeU {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EnumLongRangeU::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
#[derive(Debug, PartialEq)]
pub enum EnumLongRangeU_Constants {
    Zero,
    IntMax,
    IntOverMax,
    LongMax,
}
impl TryFrom<i64> for EnumLongRangeU_Constants {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<EnumLongRangeU_Constants> {
        match flag {
            0 => Ok(EnumLongRangeU_Constants::Zero),
            4294967295 => Ok(EnumLongRangeU_Constants::IntMax),
            4294967296 => Ok(EnumLongRangeU_Constants::IntOverMax),
            9223372036854775807 => Ok(EnumLongRangeU_Constants::LongMax),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}

