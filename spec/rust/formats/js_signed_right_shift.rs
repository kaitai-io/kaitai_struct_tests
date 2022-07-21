// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct JsSignedRightShift {
    pub should_be_40000000: Option<i32>,
    pub should_be_a00000: Option<i32>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for JsSignedRightShift {
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
impl<'r, 's: 'r> JsSignedRightShift {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = JsSignedRightShift::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn should_be_40000000<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.should_be_40000000.is_some() {
            return Ok(self.should_be_40000000.as_ref().unwrap());
        }
        self.should_be_40000000 = Some((2147483648 >> 1) as i32);
        return Ok(self.should_be_40000000.as_ref().unwrap());
    }
    fn should_be_a00000<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.should_be_a00000.is_some() {
            return Ok(self.should_be_a00000.as_ref().unwrap());
        }
        self.should_be_a00000 = Some((2684354560 >> 8) as i32);
        return Ok(self.should_be_a00000.as_ref().unwrap());
    }
}
