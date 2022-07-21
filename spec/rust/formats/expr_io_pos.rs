// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
// extraAttrForIO(RawIdentifier(NamedIdentifier(substream1)), NoRepeat)
// extraAttrForIO(RawIdentifier(NamedIdentifier(substream2)), NoRepeat)

#[derive(Default, Debug, PartialEq)]
pub struct ExprIoPos {
    pub substream1: Option<ExprIoPos_AllPlusNumber>,
    pub substream2: Option<ExprIoPos_AllPlusNumber>,
    pub raw_substream1: Vec<u8>,
    pub raw_substream2: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ExprIoPos {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.substream1 = Some(Self::read_into::<BytesReader, ExprIoPos_AllPlusNumber>(&BytesReader::new(_io.read_bytes(16 as usize)?), Some(self), _parent.push(self))?);
        self.substream2 = Some(Self::read_into::<BytesReader, ExprIoPos_AllPlusNumber>(&BytesReader::new(_io.read_bytes(14 as usize)?), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> ExprIoPos {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprIoPos::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct ExprIoPos_AllPlusNumber {
    pub my_str: String,
    pub body: Vec<u8>,
    pub number: u16,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ExprIoPos_AllPlusNumber {
    type Root = ExprIoPos;
    type ParentStack = (&'r ExprIoPos, <ExprIoPos as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.my_str = decode_string(_io.read_bytes_term(0, false, true, true)?, "UTF-8")?;
        self.body = _io.read_bytes(((_io.size - _io.pos) - 2) as usize)?.to_vec();
        self.number = _io.read_u2le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> ExprIoPos_AllPlusNumber {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprIoPos_AllPlusNumber::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
