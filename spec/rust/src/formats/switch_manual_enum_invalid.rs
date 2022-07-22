// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualEnumInvalid {
    pub opcodes: Vec<SwitchManualEnumInvalid_Opcode>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualEnumInvalid {
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
            type ArrayElement = SwitchManualEnumInvalid_Opcode;
            while !_io.is_eof() {
                self.opcodes.push(Self::read_into::<S, ArrayElement>(_io, _root, _parent.push(self))?);
            }
        }
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchManualEnumInvalid {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualEnumInvalid::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualEnumInvalid_Opcode {
    pub code: Option<SwitchManualEnumInvalid_Opcode_CodeEnum>,
    pub body: Option<SwitchManualEnumInvalid_Opcode_Body>,
}
#[derive(Debug, PartialEq)]
pub enum SwitchManualEnumInvalid_Opcode_Body {
    SwitchManualEnumInvalid_Opcode_Intval(SwitchManualEnumInvalid_Opcode_Intval),
    SwitchManualEnumInvalid_Opcode_Strval(SwitchManualEnumInvalid_Opcode_Strval),
}
impl From<SwitchManualEnumInvalid_Opcode_Intval> for SwitchManualEnumInvalid_Opcode_Body {
    fn from(v: SwitchManualEnumInvalid_Opcode_Intval) -> Self {
        Self::SwitchManualEnumInvalid_Opcode_Intval(v)
    }
}
impl From<SwitchManualEnumInvalid_Opcode_Strval> for SwitchManualEnumInvalid_Opcode_Body {
    fn from(v: SwitchManualEnumInvalid_Opcode_Strval) -> Self {
        Self::SwitchManualEnumInvalid_Opcode_Strval(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualEnumInvalid_Opcode {
    type Root = SwitchManualEnumInvalid;
    type ParentStack = (&'r SwitchManualEnumInvalid, <SwitchManualEnumInvalid as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.code = Some((_io.read_u1()? as i64).try_into()?);
        match self.code {
            SwitchManualEnumInvalid_Opcode_CodeEnum::Intval => {
                self.body = Some(Self::read_into::<S, SwitchManualEnumInvalid_Opcode_Intval>(_io, _root, _parent.push(self))?.into());
            }
            SwitchManualEnumInvalid_Opcode_CodeEnum::Strval => {
                self.body = Some(Self::read_into::<S, SwitchManualEnumInvalid_Opcode_Strval>(_io, _root, _parent.push(self))?.into());
            }
            _ => panic!("unhandled value")
        }
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchManualEnumInvalid_Opcode {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualEnumInvalid_Opcode::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
#[derive(Debug, PartialEq)]
pub enum SwitchManualEnumInvalid_Opcode_CodeEnum {
    Intval,
    Strval,
}
impl TryFrom<i64> for SwitchManualEnumInvalid_Opcode_CodeEnum {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<SwitchManualEnumInvalid_Opcode_CodeEnum> {
        match flag {
            73 => Ok(SwitchManualEnumInvalid_Opcode_CodeEnum::Intval),
            83 => Ok(SwitchManualEnumInvalid_Opcode_CodeEnum::Strval),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}


#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualEnumInvalid_Opcode_Intval {
    pub value: u8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualEnumInvalid_Opcode_Intval {
    type Root = SwitchManualEnumInvalid;
    type ParentStack = (&'r SwitchManualEnumInvalid_Opcode, <SwitchManualEnumInvalid_Opcode as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> SwitchManualEnumInvalid_Opcode_Intval {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualEnumInvalid_Opcode_Intval::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualEnumInvalid_Opcode_Strval {
    pub value: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualEnumInvalid_Opcode_Strval {
    type Root = SwitchManualEnumInvalid;
    type ParentStack = (&'r SwitchManualEnumInvalid_Opcode, <SwitchManualEnumInvalid_Opcode as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> SwitchManualEnumInvalid_Opcode_Strval {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualEnumInvalid_Opcode_Strval::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
