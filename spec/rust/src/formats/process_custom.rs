// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
// extraAttrForIO(NamedIdentifier(buf1), NoRepeat)
// extraAttrForIO(NamedIdentifier(buf2), NoRepeat)
// extraAttrForIO(NamedIdentifier(buf3), NoRepeat)

#[derive(Default, Debug, PartialEq)]
pub struct ProcessCustom {
    pub buf1: Vec<u8>,
    pub buf2: Vec<u8>,
    pub key: u8,
    pub buf3: Vec<u8>,
    pub raw_buf1: Vec<u8>,
    pub raw_buf2: Vec<u8>,
    pub raw_buf3: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ProcessCustom {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        // attrProcess(ProcessCustom(List(my_custom_fx),ArrayBuffer(IntNum(7), Bool(true), List(ArrayBuffer(IntNum(32), IntNum(48), IntNum(64))))), RawIdentifier(NamedIdentifier(buf1)), NamedIdentifier(buf1), NoRepeat)
        // attrProcess(ProcessCustom(List(nested, deeply, custom_fx),ArrayBuffer(IntNum(7))), RawIdentifier(NamedIdentifier(buf2)), NamedIdentifier(buf2), NoRepeat)
        self.key = _io.read_u1()?;
        // attrProcess(ProcessCustom(List(my_custom_fx),ArrayBuffer(Name(identifier(key)), Bool(false), List(ArrayBuffer(IntNum(0))))), RawIdentifier(NamedIdentifier(buf3)), NamedIdentifier(buf3), NoRepeat)
        Ok(())
    }
}
impl<'r, 's: 'r> ProcessCustom {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ProcessCustom::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
