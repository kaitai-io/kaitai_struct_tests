// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ExprBits {
    pub enum_seq: Option<ExprBits_Items>,
    pub a: u64,
    pub byte_size: Vec<u8>,
    pub repeat_expr: Vec<i8>,
    pub switch_on_type: Option<ExprBits_SwitchOnType>,
    pub switch_on_endian: Option<ExprBits_EndianSwitch>,
    pub enum_inst: Option<ExprBits_Items>,
    pub inst_pos: Option<i8>,
}
#[derive(Debug, PartialEq)]
pub enum ExprBits_SwitchOnType {
    S1(i8),
}
impl From<i8> for ExprBits_SwitchOnType {
    fn from(v: i8) -> Self {
        Self::S1(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for ExprBits {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.enum_seq = Some((_io.read_bits_int(2)? as i64).try_into()?);
        self.a = _io.read_bits_int(3)?;
        _io.align_to_byte()?;
        self.byte_size = _io.read_bytes(self.a as usize)?.to_vec();
        self.repeat_expr = Vec::new();
        {
            // condRepeatExprHeader(NamedIdentifier(repeat_expr), _io, Int1Type(true), Name(identifier(a)))
            // handleAssignmentRepeatExpr(NamedIdentifier(repeat_expr), _io.read_s1()?)
        }
        match self.a {
            2 => {
                self.switch_on_type = Some(_io.read_s1()?);
            }
            _ => panic!("unhandled value")
        }
        self.switch_on_endian = Some(Self::read_into::<BytesReader, ExprBits_EndianSwitch>(Self::read_into::<S, ExprBits_EndianSwitch>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> ExprBits {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprBits::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn enum_inst<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&ExprBits_Items> {
        if self.enum_inst.is_some() {
            return Ok(self.enum_inst.as_ref().unwrap());
        }
        self.enum_inst = Some(self.a as i32);
        return Ok(self.enum_inst.as_ref().unwrap());
    }
    fn inst_pos<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i8> {
        if self.inst_pos.is_some() {
            return Ok(self.inst_pos.as_ref().unwrap());
        }
        // pushPos(_io)
        // seek(_io, Name(identifier(a)))
        // popPos(_io)
        return Ok(self.inst_pos.as_ref().unwrap());
    }
}
#[derive(Debug, PartialEq)]
pub enum ExprBits_Items {
    Foo,
    Bar,
}
impl TryFrom<i64> for ExprBits_Items {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<ExprBits_Items> {
        match flag {
            1 => Ok(ExprBits_Items::Foo),
            2 => Ok(ExprBits_Items::Bar),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}


#[derive(Default, Debug, PartialEq)]
pub struct ExprBits_EndianSwitch {
    pub foo: i16,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ExprBits_EndianSwitch {
    type Root = ExprBits;
    type ParentStack = (&'r ExprBits, <ExprBits as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.foo = panic!("Unable to parse unknown-endian integers");
        Ok(())
    }
}
impl<'r, 's: 'r> ExprBits_EndianSwitch {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprBits_EndianSwitch::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
