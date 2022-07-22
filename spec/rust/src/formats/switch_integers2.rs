// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct SwitchIntegers2 {
    pub code: u8,
    pub len: Option<SwitchIntegers2_Len>,
    pub ham: Vec<u8>,
    pub padding: u8,
    pub len_mod_str: Option<String>,
}
#[derive(Debug, PartialEq)]
pub enum SwitchIntegers2_Len {
    U1(u8),
    U2(u16),
    U4(u32),
    U8(u64),
}
impl From<u8> for SwitchIntegers2_Len {
    fn from(v: u8) -> Self {
        Self::U1(v)
    }
}
impl From<u16> for SwitchIntegers2_Len {
    fn from(v: u16) -> Self {
        Self::U2(v)
    }
}
impl From<u32> for SwitchIntegers2_Len {
    fn from(v: u32) -> Self {
        Self::U4(v)
    }
}
impl From<u64> for SwitchIntegers2_Len {
    fn from(v: u64) -> Self {
        Self::U8(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for SwitchIntegers2 {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.code = _io.read_u1()?;
        match self.code {
            1 => {
                self.len = Some(_io.read_u1()?);
            }
            2 => {
                self.len = Some(_io.read_u2le()?);
            }
            4 => {
                self.len = Some(_io.read_u4le()?);
            }
            8 => {
                self.len = Some(_io.read_u8le()?);
            }
            _ => panic!("unhandled value")
        }
        self.ham = _io.read_bytes(self.len as usize)?.to_vec();
        {
            // condIfHeader(Compare(Name(identifier(len)),Gt,IntNum(3)))
            self.padding = _io.read_u1()?;
        }
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchIntegers2 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchIntegers2::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn len_mod_str<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.len_mod_str.is_some() {
            return Ok(self.len_mod_str.as_ref().unwrap());
        }
        self.len_mod_str = Some(((self.len * 2) - 1).to_string().to_string());
        return Ok(self.len_mod_str.as_ref().unwrap());
    }
}
