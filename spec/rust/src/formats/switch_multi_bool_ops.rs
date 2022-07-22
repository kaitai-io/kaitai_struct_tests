// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct SwitchMultiBoolOps {
    pub opcodes: Vec<SwitchMultiBoolOps_Opcode>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchMultiBoolOps {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.opcodes = Vec::new();
        {
            type ArrayElement = SwitchMultiBoolOps_Opcode;
            while !_io.is_eof() {
                self.opcodes.push(Self::read_into::<S, ArrayElement>(_io, _root, _parent.push(self))?);
            }
        }
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchMultiBoolOps {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchMultiBoolOps::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchMultiBoolOps_Opcode {
    pub code: u8,
    pub body: Option<SwitchMultiBoolOps_Opcode_Body>,
}
#[derive(Debug, PartialEq)]
pub enum SwitchMultiBoolOps_Opcode_Body {
    U1(u8),
    U2(u16),
    U4(u32),
    U8(u64),
}
impl From<u8> for SwitchMultiBoolOps_Opcode_Body {
    fn from(v: u8) -> Self {
        Self::U1(v)
    }
}
impl From<u16> for SwitchMultiBoolOps_Opcode_Body {
    fn from(v: u16) -> Self {
        Self::U2(v)
    }
}
impl From<u32> for SwitchMultiBoolOps_Opcode_Body {
    fn from(v: u32) -> Self {
        Self::U4(v)
    }
}
impl From<u64> for SwitchMultiBoolOps_Opcode_Body {
    fn from(v: u64) -> Self {
        Self::U8(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for SwitchMultiBoolOps_Opcode {
    type Root = SwitchMultiBoolOps;
    type ParentStack = (&'r SwitchMultiBoolOps, <SwitchMultiBoolOps as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.code = _io.read_u1()?;
        match if  ((self.code > 0) && (self.code <= 8) && (if self.code != 10 { true } else { false}))  { self.code } else { 0} {
            1 => {
                self.body = Some(_io.read_u1()?);
            }
            2 => {
                self.body = Some(_io.read_u2le()?);
            }
            4 => {
                self.body = Some(_io.read_u4le()?);
            }
            8 => {
                self.body = Some(_io.read_u8le()?);
            }
            _ => panic!("unhandled value")
        }
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchMultiBoolOps_Opcode {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchMultiBoolOps_Opcode::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
