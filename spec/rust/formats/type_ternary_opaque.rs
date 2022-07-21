// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
use crate::term_strz::TermStrz;
// extraAttrForIO(RawIdentifier(NamedIdentifier(dif_wo_hack)), NoRepeat)
// extraAttrForIO(RawIdentifier(NamedIdentifier(dif_with_hack)), NoRepeat)
// extraAttrForIO(RawIdentifier(NamedIdentifier(dif_with_hack)), NoRepeat)

#[derive(Default, Debug, PartialEq)]
pub struct TypeTernaryOpaque {
    pub dif_wo_hack: Option<Box<TermStrz>>,
    pub dif_with_hack: Option<Box<TermStrz>>,
    pub raw_dif_wo_hack: Vec<u8>,
    pub raw_dif_with_hack: Vec<u8>,
    pub raw_raw_dif_with_hack: Vec<u8>,
    pub is_hack: Option<bool>,
    pub dif: Option<Box<TermStrz>>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for TypeTernaryOpaque {
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
            self.dif_wo_hack = Some(Self::read_into::<BytesReader, Box<TermStrz>>(&BytesReader::new(_io.read_bytes(12 as usize)?), Some(self), _parent.push(self))?);
        }
        {
            // condIfHeader(Name(identifier(is_hack)))
            // attrProcess(ProcessXor(IntNum(3)), RawIdentifier(RawIdentifier(NamedIdentifier(dif_with_hack))), RawIdentifier(NamedIdentifier(dif_with_hack)), NoRepeat)
            self.dif_with_hack = Some(Self::read_into::<BytesReader, Box<TermStrz>>(&BytesReader::new(_io.read_bytes(12 as usize)?), Some(self), _parent.push(self))?);
        }
        Ok(())
    }
}
impl<'r, 's: 'r> TypeTernaryOpaque {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = TypeTernaryOpaque::default();

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
        self.is_hack = Some(false as bool);
        return Ok(self.is_hack.as_ref().unwrap());
    }
    fn dif<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&Box<TermStrz>> {
        if self.dif.is_some() {
            return Ok(self.dif.as_ref().unwrap());
        }
        self.dif = Some(if !(self.is_hack(_io, _root, _parent)?) { self.dif_wo_hack } else { self.dif_with_hack} as TermStrz);
        return Ok(self.dif.as_ref().unwrap());
    }
}
