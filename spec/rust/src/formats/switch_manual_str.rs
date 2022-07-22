// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualStr {
    pub opcodes: Vec<SwitchManualStr_Opcode>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualStr {
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
            type ArrayElement = SwitchManualStr_Opcode;
            while !_io.is_eof() {
                self.opcodes.push(Self::read_into::<S, ArrayElement>(_io, _root, _parent.push(self))?);
            }
        }
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchManualStr {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualStr::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualStr_Opcode {
    pub code: String,
    pub body: Option<SwitchManualStr_Opcode_Body>,
}
#[derive(Debug, PartialEq)]
pub enum SwitchManualStr_Opcode_Body {
    SwitchManualStr_Opcode_Intval(SwitchManualStr_Opcode_Intval),
    SwitchManualStr_Opcode_Strval(SwitchManualStr_Opcode_Strval),
}
impl From<SwitchManualStr_Opcode_Intval> for SwitchManualStr_Opcode_Body {
    fn from(v: SwitchManualStr_Opcode_Intval) -> Self {
        Self::SwitchManualStr_Opcode_Intval(v)
    }
}
impl From<SwitchManualStr_Opcode_Strval> for SwitchManualStr_Opcode_Body {
    fn from(v: SwitchManualStr_Opcode_Strval) -> Self {
        Self::SwitchManualStr_Opcode_Strval(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualStr_Opcode {
    type Root = SwitchManualStr;
    type ParentStack = (&'r SwitchManualStr, <SwitchManualStr as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.code = decode_string(_io.read_bytes(1 as usize)?, "ASCII")?;
        {
            let on = &self.code;
            if on == "I" {
                self.body = Some(Self::read_into::<S, SwitchManualStr_Opcode_Intval>(_io, _root, _parent.push(self))?.into());
            }
            else if on == "S" {
                self.body = Some(Self::read_into::<S, SwitchManualStr_Opcode_Strval>(_io, _root, _parent.push(self))?.into());
            }
        }
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchManualStr_Opcode {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualStr_Opcode::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualStr_Opcode_Intval {
    pub value: u8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualStr_Opcode_Intval {
    type Root = SwitchManualStr;
    type ParentStack = (&'r SwitchManualStr_Opcode, <SwitchManualStr_Opcode as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> SwitchManualStr_Opcode_Intval {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualStr_Opcode_Intval::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualStr_Opcode_Strval {
    pub value: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualStr_Opcode_Strval {
    type Root = SwitchManualStr;
    type ParentStack = (&'r SwitchManualStr_Opcode, <SwitchManualStr_Opcode as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> SwitchManualStr_Opcode_Strval {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualStr_Opcode_Strval::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
