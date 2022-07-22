// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct CastNested {
    pub opcodes: Vec<CastNested_Opcode>,
    pub opcodes_0_str: Option<CastNested_Opcode_Strval>,
    pub opcodes_0_str_value: Option<String>,
    pub opcodes_1_int: Option<CastNested_Opcode_Intval>,
    pub opcodes_1_int_value: Option<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for CastNested {
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
            type ArrayElement = CastNested_Opcode;
            while !_io.is_eof() {
                self.opcodes.push(Self::read_into::<S, ArrayElement>(_io, _root, _parent.push(self))?);
            }
        }
        Ok(())
    }
}
impl<'r, 's: 'r> CastNested {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = CastNested::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn opcodes_0_str<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&CastNested_Opcode_Strval> {
        if self.opcodes_0_str.is_some() {
            return Ok(self.opcodes_0_str.as_ref().unwrap());
        }
        self.opcodes_0_str = Some(self.opcodes[0 as usize].body as CastNested_Opcode_Strval);
        return Ok(self.opcodes_0_str.as_ref().unwrap());
    }
    fn opcodes_0_str_value<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.opcodes_0_str_value.is_some() {
            return Ok(self.opcodes_0_str_value.as_ref().unwrap());
        }
        self.opcodes_0_str_value = Some(self.opcodes[0 as usize].body.value.to_string());
        return Ok(self.opcodes_0_str_value.as_ref().unwrap());
    }
    fn opcodes_1_int<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&CastNested_Opcode_Intval> {
        if self.opcodes_1_int.is_some() {
            return Ok(self.opcodes_1_int.as_ref().unwrap());
        }
        self.opcodes_1_int = Some(self.opcodes[1 as usize].body as CastNested_Opcode_Intval);
        return Ok(self.opcodes_1_int.as_ref().unwrap());
    }
    fn opcodes_1_int_value<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u8> {
        if self.opcodes_1_int_value.is_some() {
            return Ok(self.opcodes_1_int_value.as_ref().unwrap());
        }
        self.opcodes_1_int_value = Some(self.opcodes[1 as usize].body.value as u8);
        return Ok(self.opcodes_1_int_value.as_ref().unwrap());
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct CastNested_Opcode {
    pub code: u8,
    pub body: Option<CastNested_Opcode_Body>,
}
#[derive(Debug, PartialEq)]
pub enum CastNested_Opcode_Body {
    CastNested_Opcode_Intval(CastNested_Opcode_Intval),
    CastNested_Opcode_Strval(CastNested_Opcode_Strval),
}
impl From<CastNested_Opcode_Intval> for CastNested_Opcode_Body {
    fn from(v: CastNested_Opcode_Intval) -> Self {
        Self::CastNested_Opcode_Intval(v)
    }
}
impl From<CastNested_Opcode_Strval> for CastNested_Opcode_Body {
    fn from(v: CastNested_Opcode_Strval) -> Self {
        Self::CastNested_Opcode_Strval(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for CastNested_Opcode {
    type Root = CastNested;
    type ParentStack = (&'r CastNested, <CastNested as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.code = _io.read_u1()?;
        match self.code {
            73 => {
                self.body = Some(Self::read_into::<S, CastNested_Opcode_Intval>(_io, _root, _parent.push(self))?.into());
            }
            83 => {
                self.body = Some(Self::read_into::<S, CastNested_Opcode_Strval>(_io, _root, _parent.push(self))?.into());
            }
            _ => panic!("unhandled value")
        }
        Ok(())
    }
}
impl<'r, 's: 'r> CastNested_Opcode {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = CastNested_Opcode::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct CastNested_Opcode_Intval {
    pub value: u8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for CastNested_Opcode_Intval {
    type Root = CastNested;
    type ParentStack = (&'r CastNested_Opcode, <CastNested_Opcode as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> CastNested_Opcode_Intval {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = CastNested_Opcode_Intval::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct CastNested_Opcode_Strval {
    pub value: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for CastNested_Opcode_Strval {
    type Root = CastNested;
    type ParentStack = (&'r CastNested_Opcode, <CastNested_Opcode as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> CastNested_Opcode_Strval {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = CastNested_Opcode_Strval::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
