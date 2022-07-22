// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
// extraAttrForIO(RawIdentifier(NamedIdentifier(envelope)), NoRepeat)

#[derive(Default, Debug, PartialEq)]
pub struct EosExceptionU4 {
    pub envelope: Option<EosExceptionU4_Data>,
    pub raw_envelope: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for EosExceptionU4 {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.envelope = Some(Self::read_into::<BytesReader, EosExceptionU4_Data>(&BytesReader::new(_io.read_bytes(6 as usize)?), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> EosExceptionU4 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EosExceptionU4::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct EosExceptionU4_Data {
    pub prebuf: Vec<u8>,
    pub fail_int: u32,
}
impl<'r, 's: 'r> KStruct<'r, 's> for EosExceptionU4_Data {
    type Root = EosExceptionU4;
    type ParentStack = (&'r EosExceptionU4, <EosExceptionU4 as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.prebuf = _io.read_bytes(3 as usize)?.to_vec();
        self.fail_int = _io.read_u4le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> EosExceptionU4_Data {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EosExceptionU4_Data::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
