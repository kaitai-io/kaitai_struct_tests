// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct SwitchCast {
    pub opcodes: Vec<SwitchCast_Opcode>,
    pub first_obj: Option<SwitchCast_Strval>,
    pub second_val: Option<u8>,
    pub err_cast: Option<SwitchCast_Strval>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchCast {
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
            type ArrayElement = SwitchCast_Opcode;
            while !_io.is_eof() {
                self.opcodes.push(Self::read_into::<S, ArrayElement>(_io, _root, _parent.push(self))?);
            }
        }
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchCast {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchCast::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn first_obj<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&SwitchCast_Strval> {
        if self.first_obj.is_some() {
            return Ok(self.first_obj.as_ref().unwrap());
        }
        self.first_obj = Some(self.opcodes[0 as usize].body as SwitchCast_Strval);
        return Ok(self.first_obj.as_ref().unwrap());
    }
    fn second_val<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u8> {
        if self.second_val.is_some() {
            return Ok(self.second_val.as_ref().unwrap());
        }
        self.second_val = Some(self.opcodes[1 as usize].body.value as u8);
        return Ok(self.second_val.as_ref().unwrap());
    }
    fn err_cast<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&SwitchCast_Strval> {
        if self.err_cast.is_some() {
            return Ok(self.err_cast.as_ref().unwrap());
        }
        self.err_cast = Some(self.opcodes[2 as usize].body as SwitchCast_Strval);
        return Ok(self.err_cast.as_ref().unwrap());
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchCast_Opcode {
    pub code: u8,
    pub body: Option<SwitchCast_Opcode_Body>,
}
#[derive(Debug, PartialEq)]
pub enum SwitchCast_Opcode_Body {
    SwitchCast_Intval(SwitchCast_Intval),
    SwitchCast_Strval(SwitchCast_Strval),
}
impl From<SwitchCast_Intval> for SwitchCast_Opcode_Body {
    fn from(v: SwitchCast_Intval) -> Self {
        Self::SwitchCast_Intval(v)
    }
}
impl From<SwitchCast_Strval> for SwitchCast_Opcode_Body {
    fn from(v: SwitchCast_Strval) -> Self {
        Self::SwitchCast_Strval(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for SwitchCast_Opcode {
    type Root = SwitchCast;
    type ParentStack = (&'r SwitchCast, <SwitchCast as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.code = _io.read_u1()?;
        match self.code {
            73 => {
                self.body = Some(Self::read_into::<S, SwitchCast_Intval>(_io, _root, _parent.push(self))?.into());
            }
            83 => {
                self.body = Some(Self::read_into::<S, SwitchCast_Strval>(_io, _root, _parent.push(self))?.into());
            }
            _ => panic!("unhandled value")
        }
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchCast_Opcode {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchCast_Opcode::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchCast_Intval {
    pub value: u8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchCast_Intval {
    type Root = SwitchCast;
    type ParentStack = (&'r SwitchCast, <SwitchCast as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> SwitchCast_Intval {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchCast_Intval::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchCast_Strval {
    pub value: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchCast_Strval {
    type Root = SwitchCast;
    type ParentStack = (&'r SwitchCast, <SwitchCast as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> SwitchCast_Strval {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchCast_Strval::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
