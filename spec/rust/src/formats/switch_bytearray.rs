// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct SwitchBytearray {
    pub opcodes: Vec<SwitchBytearray_Opcode>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchBytearray {
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
            type ArrayElement = SwitchBytearray_Opcode;
            while !_io.is_eof() {
                self.opcodes.push(Self::read_into::<S, ArrayElement>(_io, _root, _parent.push(self))?);
            }
        }
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchBytearray {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchBytearray::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchBytearray_Opcode {
    pub code: Vec<u8>,
    pub body: Option<SwitchBytearray_Opcode_Body>,
}
#[derive(Debug, PartialEq)]
pub enum SwitchBytearray_Opcode_Body {
    SwitchBytearray_Opcode_Intval(SwitchBytearray_Opcode_Intval),
    SwitchBytearray_Opcode_Strval(SwitchBytearray_Opcode_Strval),
}
impl From<SwitchBytearray_Opcode_Intval> for SwitchBytearray_Opcode_Body {
    fn from(v: SwitchBytearray_Opcode_Intval) -> Self {
        Self::SwitchBytearray_Opcode_Intval(v)
    }
}
impl From<SwitchBytearray_Opcode_Strval> for SwitchBytearray_Opcode_Body {
    fn from(v: SwitchBytearray_Opcode_Strval) -> Self {
        Self::SwitchBytearray_Opcode_Strval(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for SwitchBytearray_Opcode {
    type Root = SwitchBytearray;
    type ParentStack = (&'r SwitchBytearray, <SwitchBytearray as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.code = _io.read_bytes(1 as usize)?.to_vec();
        {
            let on = &self.code;
            if on == &[0x49] {
                self.body = Some(Self::read_into::<S, SwitchBytearray_Opcode_Intval>(_io, _root, _parent.push(self))?.into());
            }
            else if on == &[0x53] {
                self.body = Some(Self::read_into::<S, SwitchBytearray_Opcode_Strval>(_io, _root, _parent.push(self))?.into());
            }
        }
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchBytearray_Opcode {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchBytearray_Opcode::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchBytearray_Opcode_Intval {
    pub value: u8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchBytearray_Opcode_Intval {
    type Root = SwitchBytearray;
    type ParentStack = (&'r SwitchBytearray_Opcode, <SwitchBytearray_Opcode as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> SwitchBytearray_Opcode_Intval {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchBytearray_Opcode_Intval::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchBytearray_Opcode_Strval {
    pub value: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchBytearray_Opcode_Strval {
    type Root = SwitchBytearray;
    type ParentStack = (&'r SwitchBytearray_Opcode, <SwitchBytearray_Opcode as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> SwitchBytearray_Opcode_Strval {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchBytearray_Opcode_Strval::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
