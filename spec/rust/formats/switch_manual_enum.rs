// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualEnum {
    pub opcodes: Vec<SwitchManualEnum_Opcode>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualEnum {
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
            type ArrayElement = SwitchManualEnum_Opcode;
            while !_io.is_eof() {
                self.opcodes.push(Self::read_into::<S, ArrayElement>(_io, _root, _parent.push(self))?);
            }
        }
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchManualEnum {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualEnum::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualEnum_Opcode {
    pub code: Option<SwitchManualEnum_Opcode_CodeEnum>,
    pub body: Option<SwitchManualEnum_Opcode_Body>,
}
#[derive(Debug, PartialEq)]
pub enum SwitchManualEnum_Opcode_Body {
    SwitchManualEnum_Opcode_Intval(SwitchManualEnum_Opcode_Intval),
    SwitchManualEnum_Opcode_Strval(SwitchManualEnum_Opcode_Strval),
}
impl From<SwitchManualEnum_Opcode_Intval> for SwitchManualEnum_Opcode_Body {
    fn from(v: SwitchManualEnum_Opcode_Intval) -> Self {
        Self::SwitchManualEnum_Opcode_Intval(v)
    }
}
impl From<SwitchManualEnum_Opcode_Strval> for SwitchManualEnum_Opcode_Body {
    fn from(v: SwitchManualEnum_Opcode_Strval) -> Self {
        Self::SwitchManualEnum_Opcode_Strval(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualEnum_Opcode {
    type Root = SwitchManualEnum;
    type ParentStack = (&'r SwitchManualEnum, <SwitchManualEnum as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.code = Some((_io.read_u1()? as i64).try_into()?);
        match self.code {
            SwitchManualEnum_Opcode_CodeEnum::Intval => {
                self.body = Some(Self::read_into::<S, SwitchManualEnum_Opcode_Intval>(_io, _root, _parent.push(self))?.into());
            }
            SwitchManualEnum_Opcode_CodeEnum::Strval => {
                self.body = Some(Self::read_into::<S, SwitchManualEnum_Opcode_Strval>(_io, _root, _parent.push(self))?.into());
            }
            _ => panic!("unhandled value")
        }
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchManualEnum_Opcode {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualEnum_Opcode::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
#[derive(Debug, PartialEq)]
pub enum SwitchManualEnum_Opcode_CodeEnum {
    Intval,
    Strval,
}
impl TryFrom<i64> for SwitchManualEnum_Opcode_CodeEnum {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<SwitchManualEnum_Opcode_CodeEnum> {
        match flag {
            73 => Ok(SwitchManualEnum_Opcode_CodeEnum::Intval),
            83 => Ok(SwitchManualEnum_Opcode_CodeEnum::Strval),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}


#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualEnum_Opcode_Intval {
    pub value: u8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualEnum_Opcode_Intval {
    type Root = SwitchManualEnum;
    type ParentStack = (&'r SwitchManualEnum_Opcode, <SwitchManualEnum_Opcode as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> SwitchManualEnum_Opcode_Intval {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualEnum_Opcode_Intval::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualEnum_Opcode_Strval {
    pub value: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualEnum_Opcode_Strval {
    type Root = SwitchManualEnum;
    type ParentStack = (&'r SwitchManualEnum_Opcode, <SwitchManualEnum_Opcode as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> SwitchManualEnum_Opcode_Strval {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualEnum_Opcode_Strval::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
