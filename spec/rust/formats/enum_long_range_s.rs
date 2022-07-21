// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct EnumLongRangeS {
    pub f1: Option<EnumLongRangeS_Constants>,
    pub f2: Option<EnumLongRangeS_Constants>,
    pub f3: Option<EnumLongRangeS_Constants>,
    pub f4: Option<EnumLongRangeS_Constants>,
    pub f5: Option<EnumLongRangeS_Constants>,
    pub f6: Option<EnumLongRangeS_Constants>,
    pub f7: Option<EnumLongRangeS_Constants>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for EnumLongRangeS {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.f1 = Some((_io.read_s8be()? as i64).try_into()?);
        self.f2 = Some((_io.read_s8be()? as i64).try_into()?);
        self.f3 = Some((_io.read_s8be()? as i64).try_into()?);
        self.f4 = Some((_io.read_s8be()? as i64).try_into()?);
        self.f5 = Some((_io.read_s8be()? as i64).try_into()?);
        self.f6 = Some((_io.read_s8be()? as i64).try_into()?);
        self.f7 = Some((_io.read_s8be()? as i64).try_into()?);
        Ok(())
    }
}
impl<'r, 's: 'r> EnumLongRangeS {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EnumLongRangeS::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
#[derive(Debug, PartialEq)]
pub enum EnumLongRangeS_Constants {
    LongMin,
    IntBelowMin,
    IntMin,
    Zero,
    IntMax,
    IntOverMax,
    LongMax,
}
impl TryFrom<i64> for EnumLongRangeS_Constants {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<EnumLongRangeS_Constants> {
        match flag {
            -9223372036854775808 => Ok(EnumLongRangeS_Constants::LongMin),
            -2147483649 => Ok(EnumLongRangeS_Constants::IntBelowMin),
            -2147483648 => Ok(EnumLongRangeS_Constants::IntMin),
            0 => Ok(EnumLongRangeS_Constants::Zero),
            2147483647 => Ok(EnumLongRangeS_Constants::IntMax),
            2147483648 => Ok(EnumLongRangeS_Constants::IntOverMax),
            9223372036854775807 => Ok(EnumLongRangeS_Constants::LongMax),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}

