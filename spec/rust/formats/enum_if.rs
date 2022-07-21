// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct EnumIf {
    pub op1: Option<EnumIf_Operation>,
    pub op2: Option<EnumIf_Operation>,
    pub op3: Option<EnumIf_Operation>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for EnumIf {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.op1 = Some(Self::read_into::<BytesReader, EnumIf_Operation>(Self::read_into::<S, EnumIf_Operation>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.op2 = Some(Self::read_into::<BytesReader, EnumIf_Operation>(Self::read_into::<S, EnumIf_Operation>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.op3 = Some(Self::read_into::<BytesReader, EnumIf_Operation>(Self::read_into::<S, EnumIf_Operation>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> EnumIf {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EnumIf::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
#[derive(Debug, PartialEq)]
pub enum EnumIf_Opcodes {
    AString,
    ATuple,
}
impl TryFrom<i64> for EnumIf_Opcodes {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<EnumIf_Opcodes> {
        match flag {
            83 => Ok(EnumIf_Opcodes::AString),
            84 => Ok(EnumIf_Opcodes::ATuple),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}


#[derive(Default, Debug, PartialEq)]
pub struct EnumIf_Operation {
    pub opcode: Option<EnumIf_Opcodes>,
    pub arg_tuple: Option<EnumIf_ArgTuple>,
    pub arg_str: Option<EnumIf_ArgStr>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for EnumIf_Operation {
    type Root = EnumIf;
    type ParentStack = (&'r EnumIf, <EnumIf as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.opcode = Some((_io.read_u1()? as i64).try_into()?);
        {
            // condIfHeader(Compare(Name(identifier(opcode)),Eq,EnumByLabel(identifier(opcodes),identifier(a_tuple),typeId(false,List(),false))))
            self.arg_tuple = Some(Self::read_into::<BytesReader, EnumIf_ArgTuple>(Self::read_into::<S, EnumIf_ArgTuple>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        }
        {
            // condIfHeader(Compare(Name(identifier(opcode)),Eq,EnumByLabel(identifier(opcodes),identifier(a_string),typeId(false,List(),false))))
            self.arg_str = Some(Self::read_into::<BytesReader, EnumIf_ArgStr>(Self::read_into::<S, EnumIf_ArgStr>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        }
        Ok(())
    }
}
impl<'r, 's: 'r> EnumIf_Operation {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EnumIf_Operation::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct EnumIf_ArgTuple {
    pub num1: u8,
    pub num2: u8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for EnumIf_ArgTuple {
    type Root = EnumIf;
    type ParentStack = (&'r EnumIf, <EnumIf as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.num1 = _io.read_u1()?;
        self.num2 = _io.read_u1()?;
        Ok(())
    }
}
impl<'r, 's: 'r> EnumIf_ArgTuple {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EnumIf_ArgTuple::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct EnumIf_ArgStr {
    pub len: u8,
    pub str: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for EnumIf_ArgStr {
    type Root = EnumIf;
    type ParentStack = (&'r EnumIf, <EnumIf as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.len = _io.read_u1()?;
        self.str = decode_string(_io.read_bytes(self.len as usize)?, "UTF-8")?;
        Ok(())
    }
}
impl<'r, 's: 'r> EnumIf_ArgStr {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = EnumIf_ArgStr::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
