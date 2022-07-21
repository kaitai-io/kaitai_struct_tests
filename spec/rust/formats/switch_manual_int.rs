// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualInt {
    pub opcodes: Vec<SwitchManualInt_Opcode>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualInt {
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
            type ArrayElement = SwitchManualInt_Opcode;
            while !_io.is_eof() {
                self.opcodes.push(Self::read_into::<S, ArrayElement>(_io, _root, _parent.push(self))?);
            }
        }
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchManualInt {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualInt::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualInt_Opcode {
    pub code: u8,
    pub body: Option<SwitchManualInt_Opcode_Body>,
}
#[derive(Debug, PartialEq)]
pub enum SwitchManualInt_Opcode_Body {
    SwitchManualInt_Opcode_Intval(SwitchManualInt_Opcode_Intval),
    SwitchManualInt_Opcode_Strval(SwitchManualInt_Opcode_Strval),
}
impl From<SwitchManualInt_Opcode_Intval> for SwitchManualInt_Opcode_Body {
    fn from(v: SwitchManualInt_Opcode_Intval) -> Self {
        Self::SwitchManualInt_Opcode_Intval(v)
    }
}
impl From<SwitchManualInt_Opcode_Strval> for SwitchManualInt_Opcode_Body {
    fn from(v: SwitchManualInt_Opcode_Strval) -> Self {
        Self::SwitchManualInt_Opcode_Strval(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualInt_Opcode {
    type Root = SwitchManualInt;
    type ParentStack = (&'r SwitchManualInt, <SwitchManualInt as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.code = _io.read_u1()?;
        match self.code {
            73 => {
                self.body = Some(Self::read_into::<S, SwitchManualInt_Opcode_Intval>(_io, _root, _parent.push(self))?.into());
            }
            83 => {
                self.body = Some(Self::read_into::<S, SwitchManualInt_Opcode_Strval>(_io, _root, _parent.push(self))?.into());
            }
            _ => panic!("unhandled value")
        }
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchManualInt_Opcode {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualInt_Opcode::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualInt_Opcode_Intval {
    pub value: u8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualInt_Opcode_Intval {
    type Root = SwitchManualInt;
    type ParentStack = (&'r SwitchManualInt_Opcode, <SwitchManualInt_Opcode as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> SwitchManualInt_Opcode_Intval {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualInt_Opcode_Intval::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualInt_Opcode_Strval {
    pub value: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualInt_Opcode_Strval {
    type Root = SwitchManualInt;
    type ParentStack = (&'r SwitchManualInt_Opcode, <SwitchManualInt_Opcode as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.value = decode_string(_io.read_bytes_term(0, false, true, true)?, "ASCII")?;
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchManualInt_Opcode_Strval {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualInt_Opcode_Strval::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
