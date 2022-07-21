// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct NestedTypeParam {
    pub main_seq: Option<NestedTypeParam_Nested_MyType>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedTypeParam {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.main_seq = Some(Self::read_into::<BytesReader, NestedTypeParam_Nested_MyType>(Self::read_into::<S, NestedTypeParam_Nested_MyType>(5, _io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> NestedTypeParam {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedTypeParam::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NestedTypeParam_Nested {
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedTypeParam_Nested {
    type Root = NestedTypeParam;
    type ParentStack = (&'r NestedTypeParam, <NestedTypeParam as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        Ok(())
    }
}
impl<'r, 's: 'r> NestedTypeParam_Nested {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedTypeParam_Nested::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NestedTypeParam_Nested_MyType {
    pub my_len: u32,
    pub body: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedTypeParam_Nested_MyType {
    type Root = NestedTypeParam;
    type ParentStack = (&'r NestedTypeParam_Nested, <NestedTypeParam_Nested as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.body = decode_string(_io.read_bytes(self.my_len as usize)?, "ASCII")?;
        Ok(())
    }
}
impl<'r, 's: 'r> NestedTypeParam_Nested_MyType {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedTypeParam_Nested_MyType::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
