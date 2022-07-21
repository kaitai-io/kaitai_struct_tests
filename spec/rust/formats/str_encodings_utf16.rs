// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
// extraAttrForIO(RawIdentifier(NamedIdentifier(be_bom_removed)), NoRepeat)
// extraAttrForIO(RawIdentifier(NamedIdentifier(le_bom_removed)), NoRepeat)

#[derive(Default, Debug, PartialEq)]
pub struct StrEncodingsUtf16 {
    pub len_be: u32,
    pub be_bom_removed: Option<StrEncodingsUtf16_StrBeBomRemoved>,
    pub len_le: u32,
    pub le_bom_removed: Option<StrEncodingsUtf16_StrLeBomRemoved>,
    pub raw_be_bom_removed: Vec<u8>,
    pub raw_le_bom_removed: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for StrEncodingsUtf16 {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.len_be = _io.read_u4le()?;
        self.be_bom_removed = Some(Self::read_into::<BytesReader, StrEncodingsUtf16_StrBeBomRemoved>(&BytesReader::new(_io.read_bytes(self.len_be as usize)?), Some(self), _parent.push(self))?);
        self.len_le = _io.read_u4le()?;
        self.le_bom_removed = Some(Self::read_into::<BytesReader, StrEncodingsUtf16_StrLeBomRemoved>(&BytesReader::new(_io.read_bytes(self.len_le as usize)?), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> StrEncodingsUtf16 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = StrEncodingsUtf16::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct StrEncodingsUtf16_StrBeBomRemoved {
    pub bom: u16,
    pub str: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for StrEncodingsUtf16_StrBeBomRemoved {
    type Root = StrEncodingsUtf16;
    type ParentStack = (&'r StrEncodingsUtf16, <StrEncodingsUtf16 as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.bom = _io.read_u2be()?;
        self.str = decode_string(_io.read_bytes_full()?, "UTF-16BE")?;
        Ok(())
    }
}
impl<'r, 's: 'r> StrEncodingsUtf16_StrBeBomRemoved {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = StrEncodingsUtf16_StrBeBomRemoved::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct StrEncodingsUtf16_StrLeBomRemoved {
    pub bom: u16,
    pub str: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for StrEncodingsUtf16_StrLeBomRemoved {
    type Root = StrEncodingsUtf16;
    type ParentStack = (&'r StrEncodingsUtf16, <StrEncodingsUtf16 as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.bom = _io.read_u2le()?;
        self.str = decode_string(_io.read_bytes_full()?, "UTF-16LE")?;
        Ok(())
    }
}
impl<'r, 's: 'r> StrEncodingsUtf16_StrLeBomRemoved {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = StrEncodingsUtf16_StrLeBomRemoved::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
