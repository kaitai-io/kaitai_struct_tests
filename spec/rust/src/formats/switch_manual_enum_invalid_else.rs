// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualEnumInvalidElse {
    pub opcodes: Vec<SwitchManualEnumInvalidElse_Opcode>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualEnumInvalidElse {
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
            type ArrayElement = SwitchManualEnumInvalidElse_Opcode;
            while !_io.is_eof() {
                self.opcodes.push(Self::read_into::<S, ArrayElement>(_io, _root, _parent.push(self))?);
            }
        }
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchManualEnumInvalidElse {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualEnumInvalidElse::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualEnumInvalidElse_Opcode {
    pub code: Option<SwitchManualEnumInvalidElse_Opcode_CodeEnum>,
    pub body: Option<SwitchManualEnumInvalidElse_Opcode_Body>,
}
#[derive(Debug, PartialEq)]
pub enum SwitchManualEnumInvalidElse_Opcode_Body {
    SwitchManualEnumInvalidElse_Opcode_Intval(SwitchManualEnumInvalidElse_Opcode_Intval),
    SwitchManualEnumInvalidElse_Opcode_Strval(SwitchManualEnumInvalidElse_Opcode_Strval),
    SwitchManualEnumInvalidElse_Opcode_Defval(SwitchManualEnumInvalidElse_Opcode_Defval),
}
impl From<SwitchManualEnumInvalidElse_Opcode_Intval> for SwitchManualEnumInvalidElse_Opcode_Body {
    fn from(v: SwitchManualEnumInvalidElse_Opcode_Intval) -> Self {
        Self::SwitchManualEnumInvalidElse_Opcode_Intval(v)
    }
}
impl From<SwitchManualEnumInvalidElse_Opcode_Strval> for SwitchManualEnumInvalidElse_Opcode_Body {
    fn from(v: SwitchManualEnumInvalidElse_Opcode_Strval) -> Self {
        Self::SwitchManualEnumInvalidElse_Opcode_Strval(v)
    }
}
impl From<SwitchManualEnumInvalidElse_Opcode_Defval> for SwitchManualEnumInvalidElse_Opcode_Body {
    fn from(v: SwitchManualEnumInvalidElse_Opcode_Defval) -> Self {
        Self::SwitchManualEnumInvalidElse_Opcode_Defval(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualEnumInvalidElse_Opcode {
    type Root = SwitchManualEnumInvalidElse;
    type ParentStack = (&'r SwitchManualEnumInvalidElse, <SwitchManualEnumInvalidElse as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.code = Some((_io.read_u1()? as i64).try_into()?);
        match self.code {
            SwitchManualEnumInvalidElse_Opcode_CodeEnum::Intval => {
                self.body = Some(Self::read_into::<S, SwitchManualEnumInvalidElse_Opcode_Intval>(_io, _root, _parent.push(self))?.into());
            }
            SwitchManualEnumInvalidElse_Opcode_CodeEnum::Strval => {
                self.body = Some(Self::read_into::<S, SwitchManualEnumInvalidElse_Opcode_Strval>(_io, _root, _parent.push(self))?.into());
            }
            // switchElseStart()
            self.body = Some(Self::read_into::<S, SwitchManualEnumInvalidElse_Opcode_Defval>(_io, _root, _parent.push(self))?.into());
        }
        _ => panic!("unhandled value")
    }
    Ok(())
}
}
impl<'r, 's: 'r> SwitchManualEnumInvalidElse_Opcode {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualEnumInvalidElse_Opcode::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
#[derive(Debug, PartialEq)]
pub enum SwitchManualEnumInvalidElse_Opcode_CodeEnum {
Intval,
Strval,
}
impl TryFrom<i64> for SwitchManualEnumInvalidElse_Opcode_CodeEnum {
type Error = KError;
fn try_from(flag: i64) -> KResult<SwitchManualEnumInvalidElse_Opcode_CodeEnum> {
    match flag {
        73 => Ok(SwitchManualEnumInvalidElse_Opcode_CodeEnum::Intval),
        83 => Ok(SwitchManualEnumInvalidElse_Opcode_CodeEnum::Strval),
        _ => Err(KError::UnknownVariant(flag)),
    }
}
}


#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualEnumInvalidElse_Opcode_Intval {
pub value: u8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualEnumInvalidElse_Opcode_Intval {
type Root = SwitchManualEnumInvalidElse;
type ParentStack = (&'r SwitchManualEnumInvalidElse_Opcode, <SwitchManualEnumInvalidElse_Opcode as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> SwitchManualEnumInvalidElse_Opcode_Intval {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualEnumInvalidElse_Opcode_Intval::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualEnumInvalidElse_Opcode_Strval {
pub value: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualEnumInvalidElse_Opcode_Strval {
type Root = SwitchManualEnumInvalidElse;
type ParentStack = (&'r SwitchManualEnumInvalidElse_Opcode, <SwitchManualEnumInvalidElse_Opcode as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> SwitchManualEnumInvalidElse_Opcode_Strval {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualEnumInvalidElse_Opcode_Strval::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualEnumInvalidElse_Opcode_Defval {
pub value: Option<i8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualEnumInvalidElse_Opcode_Defval {
type Root = SwitchManualEnumInvalidElse;
type ParentStack = (&'r SwitchManualEnumInvalidElse_Opcode, <SwitchManualEnumInvalidElse_Opcode as KStruct<'r, 's>>::ParentStack);

fn read<S: KStream>(
    &mut self,
    _io: &'s S,
    _root: Option<&'r Self::Root>,
    _parent: Option<TypedStack<Self::ParentStack>>
) -> KResult<()> {
    Ok(())
}
}
impl<'r, 's: 'r> SwitchManualEnumInvalidElse_Opcode_Defval {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualEnumInvalidElse_Opcode_Defval::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

fn value<S: KStream>(
    &mut self,
    _io: &'s S,
    _root: Option<&'r SwitchManualEnumInvalidElse>,
    _parent: Option<TypedStack<(&'r SwitchManualEnumInvalidElse_Opcode, <SwitchManualEnumInvalidElse_Opcode as KStruct<'r, 's>>::ParentStack)>>
) -> KResult<&i8> {
    if self.value.is_some() {
        return Ok(self.value.as_ref().unwrap());
    }
    self.value = Some(123 as i8);
    return Ok(self.value.as_ref().unwrap());
}
}
