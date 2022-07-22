// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
// extraAttrForIO(RawIdentifier(NamedIdentifier(dif_wo_hack)), NoRepeat)
// extraAttrForIO(RawIdentifier(NamedIdentifier(dif_with_hack)), NoRepeat)
// extraAttrForIO(RawIdentifier(NamedIdentifier(dif_with_hack)), NoRepeat)

#[derive(Default, Debug, PartialEq)]
pub struct TypeTernary {
    pub dif_wo_hack: Option<TypeTernary_Dummy>,
    pub dif_with_hack: Option<TypeTernary_Dummy>,
    pub raw_dif_wo_hack: Vec<u8>,
    pub raw_dif_with_hack: Vec<u8>,
    pub raw_raw_dif_with_hack: Vec<u8>,
    pub is_hack: Option<bool>,
    pub dif: Option<TypeTernary_Dummy>,
    pub dif_value: Option<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for TypeTernary {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        {
            // condIfHeader(UnaryOp(Not,Name(identifier(is_hack))))
            self.dif_wo_hack = Some(Self::read_into::<BytesReader, TypeTernary_Dummy>(&BytesReader::new(_io.read_bytes(1 as usize)?), Some(self), _parent.push(self))?);
        }
        // attrProcess(ProcessXor(IntNum(3)), RawIdentifier(RawIdentifier(NamedIdentifier(dif_with_hack))), RawIdentifier(NamedIdentifier(dif_with_hack)), NoRepeat)
        self.dif_with_hack = Some(Self::read_into::<BytesReader, TypeTernary_Dummy>(&BytesReader::new(_io.read_bytes(1 as usize)?), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> TypeTernary {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = TypeTernary::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn is_hack<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.is_hack.is_some() {
            return Ok(self.is_hack.as_ref().unwrap());
        }
        self.is_hack = Some(true as bool);
        return Ok(self.is_hack.as_ref().unwrap());
    }
    fn dif<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&TypeTernary_Dummy> {
        if self.dif.is_some() {
            return Ok(self.dif.as_ref().unwrap());
        }
        self.dif = Some(if !(self.is_hack(_io, _root, _parent)?) { self.dif_wo_hack } else { self.dif_with_hack} as Dummy);
        return Ok(self.dif.as_ref().unwrap());
    }
    fn dif_value<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u8> {
        if self.dif_value.is_some() {
            return Ok(self.dif_value.as_ref().unwrap());
        }
        self.dif_value = Some(self.dif(_io, _root, _parent)?.value as u8);
        return Ok(self.dif_value.as_ref().unwrap());
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct TypeTernary_Dummy {
    pub value: u8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for TypeTernary_Dummy {
    type Root = TypeTernary;
    type ParentStack = (&'r TypeTernary, <TypeTernary as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.value = _io.read_u1()?;
        Ok(())
    }
}
impl<'r, 's: 'r> TypeTernary_Dummy {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = TypeTernary_Dummy::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
