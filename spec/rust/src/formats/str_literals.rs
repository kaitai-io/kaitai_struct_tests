// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct StrLiterals {
    pub octal_eatup2: Option<String>,
    pub backslashes: Option<String>,
    pub octal_eatup: Option<String>,
    pub double_quotes: Option<String>,
    pub complex_str: Option<String>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for StrLiterals {
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
impl<'r, 's: 'r> StrLiterals {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = StrLiterals::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn octal_eatup2<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.octal_eatup2.is_some() {
            return Ok(self.octal_eatup2.as_ref().unwrap());
        }
        self.octal_eatup2 = Some("\u{2}2".to_string());
        return Ok(self.octal_eatup2.as_ref().unwrap());
    }
    fn backslashes<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.backslashes.is_some() {
            return Ok(self.backslashes.as_ref().unwrap());
        }
        self.backslashes = Some("\\\\\\".to_string());
        return Ok(self.backslashes.as_ref().unwrap());
    }
    fn octal_eatup<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.octal_eatup.is_some() {
            return Ok(self.octal_eatup.as_ref().unwrap());
        }
        self.octal_eatup = Some("\u{0}22".to_string());
        return Ok(self.octal_eatup.as_ref().unwrap());
    }
    fn double_quotes<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.double_quotes.is_some() {
            return Ok(self.double_quotes.as_ref().unwrap());
        }
        self.double_quotes = Some("\"\"\"".to_string());
        return Ok(self.double_quotes.as_ref().unwrap());
    }
    fn complex_str<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.complex_str.is_some() {
            return Ok(self.complex_str.as_ref().unwrap());
        }
        self.complex_str = Some("\u{0}\u{1}\u{2}\u{7}\u{8}\n\r\t\u{b}\u{c}\u{1b}=\u{7}\n$\u{263b}".to_string());
        return Ok(self.complex_str.as_ref().unwrap());
    }
}
