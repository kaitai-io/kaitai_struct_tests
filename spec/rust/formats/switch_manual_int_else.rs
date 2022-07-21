// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualIntElse {
    pub opcodes: Vec<SwitchManualIntElse_Opcode>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualIntElse {
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
            type ArrayElement = SwitchManualIntElse_Opcode;
            while !_io.is_eof() {
                self.opcodes.push(Self::read_into::<S, ArrayElement>(_io, _root, _parent.push(self))?);
            }
        }
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchManualIntElse {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualIntElse::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualIntElse_Opcode {
    pub code: u8,
    pub body: Option<SwitchManualIntElse_Opcode_Body>,
}
#[derive(Debug, PartialEq)]
pub enum SwitchManualIntElse_Opcode_Body {
    SwitchManualIntElse_Opcode_Intval(SwitchManualIntElse_Opcode_Intval),
    SwitchManualIntElse_Opcode_Strval(SwitchManualIntElse_Opcode_Strval),
    SwitchManualIntElse_Opcode_Noneval(SwitchManualIntElse_Opcode_Noneval),
}
impl From<SwitchManualIntElse_Opcode_Intval> for SwitchManualIntElse_Opcode_Body {
    fn from(v: SwitchManualIntElse_Opcode_Intval) -> Self {
        Self::SwitchManualIntElse_Opcode_Intval(v)
    }
}
impl From<SwitchManualIntElse_Opcode_Strval> for SwitchManualIntElse_Opcode_Body {
    fn from(v: SwitchManualIntElse_Opcode_Strval) -> Self {
        Self::SwitchManualIntElse_Opcode_Strval(v)
    }
}
impl From<SwitchManualIntElse_Opcode_Noneval> for SwitchManualIntElse_Opcode_Body {
    fn from(v: SwitchManualIntElse_Opcode_Noneval) -> Self {
        Self::SwitchManualIntElse_Opcode_Noneval(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualIntElse_Opcode {
    type Root = SwitchManualIntElse;
    type ParentStack = (&'r SwitchManualIntElse, <SwitchManualIntElse as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.code = _io.read_u1()?;
        match self.code {
            73 => {
                self.body = Some(Self::read_into::<S, SwitchManualIntElse_Opcode_Intval>(_io, _root, _parent.push(self))?.into());
            }
            83 => {
                self.body = Some(Self::read_into::<S, SwitchManualIntElse_Opcode_Strval>(_io, _root, _parent.push(self))?.into());
            }
            // switchElseStart()
            self.body = Some(Self::read_into::<S, SwitchManualIntElse_Opcode_Noneval>(_io, _root, _parent.push(self))?.into());
        }
        _ => panic!("unhandled value")
    }
    Ok(())
}
}
impl<'r, 's: 'r> SwitchManualIntElse_Opcode {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualIntElse_Opcode::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualIntElse_Opcode_Intval {
pub value: u8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualIntElse_Opcode_Intval {
type Root = SwitchManualIntElse;
type ParentStack = (&'r SwitchManualIntElse_Opcode, <SwitchManualIntElse_Opcode as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> SwitchManualIntElse_Opcode_Intval {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualIntElse_Opcode_Intval::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualIntElse_Opcode_Strval {
pub value: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualIntElse_Opcode_Strval {
type Root = SwitchManualIntElse;
type ParentStack = (&'r SwitchManualIntElse_Opcode, <SwitchManualIntElse_Opcode as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> SwitchManualIntElse_Opcode_Strval {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualIntElse_Opcode_Strval::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualIntElse_Opcode_Noneval {
pub filler: u32,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualIntElse_Opcode_Noneval {
type Root = SwitchManualIntElse;
type ParentStack = (&'r SwitchManualIntElse_Opcode, <SwitchManualIntElse_Opcode as KStruct<'r, 's>>::ParentStack);

fn read<S: KStream>(
    &mut self,
    _io: &'s S,
    _root: Option<&'r Self::Root>,
    _parent: Option<TypedStack<Self::ParentStack>>
) -> KResult<()> {
    self.filler = _io.read_u4le()?;
    Ok(())
}
}
impl<'r, 's: 'r> SwitchManualIntElse_Opcode_Noneval {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualIntElse_Opcode_Noneval::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
