// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
// extraAttrForIO(RawIdentifier(NamedIdentifier(envelope)), NoRepeat)

#[derive(Default, Debug, PartialEq)]
pub struct EosExceptionBytes {
    pub envelope: Option<EosExceptionBytes_Data>,
    pub raw_envelope: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for EosExceptionBytes {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.envelope = Some(Self::read_into::<BytesReader, EosExceptionBytes_Data>(&BytesReader::new(_io.read_bytes(6 as usize)?), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> EosExceptionBytes {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EosExceptionBytes::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct EosExceptionBytes_Data {
    pub buf: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for EosExceptionBytes_Data {
    type Root = EosExceptionBytes;
    type ParentStack = (&'r EosExceptionBytes, <EosExceptionBytes as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.buf = _io.read_bytes(7 as usize)?.to_vec();
        Ok(())
    }
}
impl<'r, 's: 'r> EosExceptionBytes_Data {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EosExceptionBytes_Data::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
