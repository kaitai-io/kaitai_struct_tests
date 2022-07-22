// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct RepeatUntilComplex {
    pub first: Vec<RepeatUntilComplex_TypeU1>,
    pub second: Vec<RepeatUntilComplex_TypeU2>,
    pub third: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for RepeatUntilComplex {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.first = Vec::new();
        {
            // condRepeatUntilHeader(NamedIdentifier(first), _io, UserTypeInstream(List(type_u1),None,List()), Compare(Attribute(Name(identifier(_)),identifier(count)),Eq,IntNum(0)))
            // handleAssignmentRepeatUntil(NamedIdentifier(first), Self::read_into::<S, RepeatUntilComplex_TypeU1>(_io, _root, _parent.push(self))?.into(), false)
            // condRepeatUntilFooter(NamedIdentifier(first), _io, UserTypeInstream(List(type_u1),None,List()), Compare(Attribute(Name(identifier(_)),identifier(count)),Eq,IntNum(0)))
        } {}
        self.second = Vec::new();
        {
            // condRepeatUntilHeader(NamedIdentifier(second), _io, UserTypeInstream(List(type_u2),None,List()), Compare(Attribute(Name(identifier(_)),identifier(count)),Eq,IntNum(0)))
            // handleAssignmentRepeatUntil(NamedIdentifier(second), Self::read_into::<S, RepeatUntilComplex_TypeU2>(_io, _root, _parent.push(self))?.into(), false)
            // condRepeatUntilFooter(NamedIdentifier(second), _io, UserTypeInstream(List(type_u2),None,List()), Compare(Attribute(Name(identifier(_)),identifier(count)),Eq,IntNum(0)))
        } {}
        self.third = Vec::new();
        {
            // condRepeatUntilHeader(NamedIdentifier(third), _io, Int1Type(false), Compare(Name(identifier(_)),Eq,IntNum(0)))
            // handleAssignmentRepeatUntil(NamedIdentifier(third), _io.read_u1()?, false)
            // condRepeatUntilFooter(NamedIdentifier(third), _io, Int1Type(false), Compare(Name(identifier(_)),Eq,IntNum(0)))
        } {}
        Ok(())
    }
}
impl<'r, 's: 'r> RepeatUntilComplex {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = RepeatUntilComplex::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct RepeatUntilComplex_TypeU1 {
    pub count: u8,
    pub values: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for RepeatUntilComplex_TypeU1 {
    type Root = RepeatUntilComplex;
    type ParentStack = (&'r RepeatUntilComplex, <RepeatUntilComplex as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.count = _io.read_u1()?;
        self.values = Vec::new();
        {
            // condRepeatExprHeader(NamedIdentifier(values), _io, Int1Type(false), Name(identifier(count)))
            // handleAssignmentRepeatExpr(NamedIdentifier(values), _io.read_u1()?)
        }
        Ok(())
    }
}
impl<'r, 's: 'r> RepeatUntilComplex_TypeU1 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = RepeatUntilComplex_TypeU1::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct RepeatUntilComplex_TypeU2 {
    pub count: u16,
    pub values: Vec<u16>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for RepeatUntilComplex_TypeU2 {
    type Root = RepeatUntilComplex;
    type ParentStack = (&'r RepeatUntilComplex, <RepeatUntilComplex as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.count = _io.read_u2le()?;
        self.values = Vec::new();
        {
            // condRepeatExprHeader(NamedIdentifier(values), _io, IntMultiType(false,Width2,Some(LittleEndian)), Name(identifier(count)))
            // handleAssignmentRepeatExpr(NamedIdentifier(values), _io.read_u2le()?)
        }
        Ok(())
    }
}
impl<'r, 's: 'r> RepeatUntilComplex_TypeU2 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = RepeatUntilComplex_TypeU2::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
