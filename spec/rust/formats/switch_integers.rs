// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct SwitchIntegers {
    pub opcodes: Vec<SwitchIntegers_Opcode>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchIntegers {
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
            type ArrayElement = SwitchIntegers_Opcode;
            while !_io.is_eof() {
                self.opcodes.push(Self::read_into::<S, ArrayElement>(_io, _root, _parent.push(self))?);
            }
        }
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchIntegers {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchIntegers::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchIntegers_Opcode {
    pub code: u8,
    pub body: Option<SwitchIntegers_Opcode_Body>,
}
#[derive(Debug, PartialEq)]
pub enum SwitchIntegers_Opcode_Body {
    U1(u8),
    U2(u16),
    U4(u32),
    U8(u64),
}
impl From<u8> for SwitchIntegers_Opcode_Body {
    fn from(v: u8) -> Self {
        Self::U1(v)
    }
}
impl From<u16> for SwitchIntegers_Opcode_Body {
    fn from(v: u16) -> Self {
        Self::U2(v)
    }
}
impl From<u32> for SwitchIntegers_Opcode_Body {
    fn from(v: u32) -> Self {
        Self::U4(v)
    }
}
impl From<u64> for SwitchIntegers_Opcode_Body {
    fn from(v: u64) -> Self {
        Self::U8(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for SwitchIntegers_Opcode {
    type Root = SwitchIntegers;
    type ParentStack = (&'r SwitchIntegers, <SwitchIntegers as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.code = _io.read_u1()?;
        match self.code {
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
impl<'r, 's: 'r> SwitchIntegers_Opcode {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchIntegers_Opcode::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
