// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualStrElse {
    pub opcodes: Vec<SwitchManualStrElse_Opcode>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualStrElse {
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
            type ArrayElement = SwitchManualStrElse_Opcode;
            while !_io.is_eof() {
                self.opcodes.push(Self::read_into::<S, ArrayElement>(_io, _root, _parent.push(self))?);
            }
        }
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchManualStrElse {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualStrElse::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualStrElse_Opcode {
    pub code: String,
    pub body: Option<SwitchManualStrElse_Opcode_Body>,
}
#[derive(Debug, PartialEq)]
pub enum SwitchManualStrElse_Opcode_Body {
    SwitchManualStrElse_Opcode_Intval(SwitchManualStrElse_Opcode_Intval),
    SwitchManualStrElse_Opcode_Strval(SwitchManualStrElse_Opcode_Strval),
    SwitchManualStrElse_Opcode_Noneval(SwitchManualStrElse_Opcode_Noneval),
}
impl From<SwitchManualStrElse_Opcode_Intval> for SwitchManualStrElse_Opcode_Body {
    fn from(v: SwitchManualStrElse_Opcode_Intval) -> Self {
        Self::SwitchManualStrElse_Opcode_Intval(v)
    }
}
impl From<SwitchManualStrElse_Opcode_Strval> for SwitchManualStrElse_Opcode_Body {
    fn from(v: SwitchManualStrElse_Opcode_Strval) -> Self {
        Self::SwitchManualStrElse_Opcode_Strval(v)
    }
}
impl From<SwitchManualStrElse_Opcode_Noneval> for SwitchManualStrElse_Opcode_Body {
    fn from(v: SwitchManualStrElse_Opcode_Noneval) -> Self {
        Self::SwitchManualStrElse_Opcode_Noneval(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualStrElse_Opcode {
    type Root = SwitchManualStrElse;
    type ParentStack = (&'r SwitchManualStrElse, <SwitchManualStrElse as KStruct<'r, 's>>::ParentStack);

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
                self.body = Some(Self::read_into::<S, SwitchManualStrElse_Opcode_Intval>(_io, _root, _parent.push(self))?.into());
            }
            else if on == "S" {
                self.body = Some(Self::read_into::<S, SwitchManualStrElse_Opcode_Strval>(_io, _root, _parent.push(self))?.into());
            }
            else {
                self.body = Some(Self::read_into::<S, SwitchManualStrElse_Opcode_Noneval>(_io, _root, _parent.push(self))?.into());
            }
        }
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchManualStrElse_Opcode {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualStrElse_Opcode::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualStrElse_Opcode_Intval {
    pub value: u8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualStrElse_Opcode_Intval {
    type Root = SwitchManualStrElse;
    type ParentStack = (&'r SwitchManualStrElse_Opcode, <SwitchManualStrElse_Opcode as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> SwitchManualStrElse_Opcode_Intval {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualStrElse_Opcode_Intval::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualStrElse_Opcode_Strval {
    pub value: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualStrElse_Opcode_Strval {
    type Root = SwitchManualStrElse;
    type ParentStack = (&'r SwitchManualStrElse_Opcode, <SwitchManualStrElse_Opcode as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> SwitchManualStrElse_Opcode_Strval {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualStrElse_Opcode_Strval::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualStrElse_Opcode_Noneval {
    pub filler: u32,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualStrElse_Opcode_Noneval {
    type Root = SwitchManualStrElse;
    type ParentStack = (&'r SwitchManualStrElse_Opcode, <SwitchManualStrElse_Opcode as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> SwitchManualStrElse_Opcode_Noneval {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualStrElse_Opcode_Noneval::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
