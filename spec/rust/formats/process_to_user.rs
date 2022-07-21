// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
// extraAttrForIO(RawIdentifier(NamedIdentifier(buf1)), NoRepeat)
// extraAttrForIO(RawIdentifier(NamedIdentifier(buf1)), NoRepeat)

#[derive(Default, Debug, PartialEq)]
pub struct ProcessToUser {
    pub buf1: Option<ProcessToUser_JustStr>,
    pub raw_buf1: Vec<u8>,
    pub raw_raw_buf1: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ProcessToUser {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        // attrProcess(ProcessRotate(true,IntNum(3)), RawIdentifier(RawIdentifier(NamedIdentifier(buf1))), RawIdentifier(NamedIdentifier(buf1)), NoRepeat)
        self.buf1 = Some(Self::read_into::<BytesReader, ProcessToUser_JustStr>(&BytesReader::new(_io.read_bytes(5 as usize)?), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> ProcessToUser {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ProcessToUser::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct ProcessToUser_JustStr {
    pub str: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ProcessToUser_JustStr {
    type Root = ProcessToUser;
    type ParentStack = (&'r ProcessToUser, <ProcessToUser as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.str = decode_string(_io.read_bytes_full()?, "UTF-8")?;
        Ok(())
    }
}
impl<'r, 's: 'r> ProcessToUser_JustStr {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ProcessToUser_JustStr::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
