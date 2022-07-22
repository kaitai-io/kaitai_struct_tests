// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct IfValues {
    pub codes: Vec<IfValues_Code>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for IfValues {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.codes = Vec::new();
        {
            // condRepeatExprHeader(NamedIdentifier(codes), _io, UserTypeInstream(List(code),None,List()), IntNum(3))
            // handleAssignmentRepeatExpr(NamedIdentifier(codes), Self::read_into::<S, IfValues_Code>(_io, _root, _parent.push(self))?.into())
        }
        Ok(())
    }
}
impl<'r, 's: 'r> IfValues {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = IfValues::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct IfValues_Code {
    pub opcode: u8,
    pub half_opcode: Option<i32>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for IfValues_Code {
    type Root = IfValues;
    type ParentStack = (&'r IfValues, <IfValues as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.opcode = _io.read_u1()?;
        Ok(())
    }
}
impl<'r, 's: 'r> IfValues_Code {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = IfValues_Code::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn half_opcode<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r IfValues>,
        _parent: Option<TypedStack<(&'r IfValues, <IfValues as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.half_opcode.is_some() {
            return Ok(self.half_opcode.as_ref().unwrap());
        }
        {
            // condIfHeader(Compare(BinOp(Name(identifier(opcode)),Mod,IntNum(2)),Eq,IntNum(0)))
            self.half_opcode = Some(self.opcode / 2 as i32);
        }
        return Ok(self.half_opcode.as_ref().unwrap());
    }
}
