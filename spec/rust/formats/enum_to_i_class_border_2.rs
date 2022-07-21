// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
use crate::enum_to_i_class_border_1::EnumToIClassBorder1;

#[derive(Default, Debug, PartialEq)]
pub struct EnumToIClassBorder2 {
    pub parent: Option<Box<EnumToIClassBorder1>>,
    pub is_dog: Option<bool>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for EnumToIClassBorder2 {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        Ok(())
    }
}
impl<'r, 's: 'r> EnumToIClassBorder2 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EnumToIClassBorder2::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn is_dog<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.is_dog.is_some() {
            return Ok(self.is_dog.as_ref().unwrap());
        }
        self.is_dog = Some(self.parent.some_dog == 4 as bool);
        return Ok(self.is_dog.as_ref().unwrap());
    }
}
