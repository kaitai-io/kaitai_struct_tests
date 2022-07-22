// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
// extraAttrForIO(RawIdentifier(NamedIdentifier(body)), RepeatExpr(IntNum(1)))
// extraAttrForIO(RawIdentifier(NamedIdentifier(body)), RepeatExpr(IntNum(1)))

#[derive(Default, Debug, PartialEq)]
pub struct SwitchRepeatExprInvalid {
    pub code: u8,
    pub size: u32,
    pub body: Vec<SwitchRepeatExprInvalid_Body>,
    pub raw_body: Vec<Vec<u8>>,
}
#[derive(Debug, PartialEq)]
pub enum SwitchRepeatExprInvalid_Body {
    SwitchRepeatExprInvalid_One(SwitchRepeatExprInvalid_One),
    SwitchRepeatExprInvalid_Two(SwitchRepeatExprInvalid_Two),
    Bytes(Vec<u8>),
}
impl From<SwitchRepeatExprInvalid_One> for SwitchRepeatExprInvalid_Body {
    fn from(v: SwitchRepeatExprInvalid_One) -> Self {
        Self::SwitchRepeatExprInvalid_One(v)
    }
}
impl From<SwitchRepeatExprInvalid_Two> for SwitchRepeatExprInvalid_Body {
    fn from(v: SwitchRepeatExprInvalid_Two) -> Self {
        Self::SwitchRepeatExprInvalid_Two(v)
    }
}
impl From<Vec<u8>> for SwitchRepeatExprInvalid_Body {
    fn from(v: Vec<u8>) -> Self {
        Self::Bytes(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for SwitchRepeatExprInvalid {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.code = _io.read_u1()?;
        self.size = _io.read_u4le()?;
        self.body = Vec::new();
        {
            // condRepeatExprHeader(NamedIdentifier(body), _io, SwitchType(Name(identifier(code)),Map(IntNum(255) -> UserTypeFromBytes(List(one),None,List(),BytesLimitType(Name(identifier(size)),None,false,None,None),None), IntNum(34) -> UserTypeFromBytes(List(two),None,List(),BytesLimitType(Name(identifier(size)),None,false,None,None),None), Name(identifier(_)) -> BytesLimitType(Name(identifier(size)),None,false,None,None)),true), IntNum(1))
            match self.code {
                255 => {
                    // handleAssignmentRepeatExpr(RawIdentifier(NamedIdentifier(body)), _io.read_bytes(self.size as usize)?)
                    // handleAssignmentRepeatExpr(NamedIdentifier(body), &BytesReader::new(_io.read_bytes(self.size as usize)?))
                }
                34 => {
                    // handleAssignmentRepeatExpr(RawIdentifier(NamedIdentifier(body)), _io.read_bytes(self.size as usize)?)
                    // handleAssignmentRepeatExpr(NamedIdentifier(body), &BytesReader::new(_io.read_bytes(self.size as usize)?))
                }
                // switchElseStart()
                // handleAssignmentRepeatExpr(NamedIdentifier(body), _io.read_bytes(self.size as usize)?)
            }
            _ => panic!("unhandled value")
        }
    }
    Ok(())
}
}
impl<'r, 's: 'r> SwitchRepeatExprInvalid {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchRepeatExprInvalid::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchRepeatExprInvalid_One {
pub first: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchRepeatExprInvalid_One {
type Root = SwitchRepeatExprInvalid;
type ParentStack = (&'r SwitchRepeatExprInvalid, <SwitchRepeatExprInvalid as KStruct<'r, 's>>::ParentStack);

fn read<S: KStream>(
    &mut self,
    _io: &'s S,
    _root: Option<&'r Self::Root>,
    _parent: Option<TypedStack<Self::ParentStack>>
) -> KResult<()> {
    self.first = _io.read_bytes_full()?;
    Ok(())
}
}
impl<'r, 's: 'r> SwitchRepeatExprInvalid_One {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchRepeatExprInvalid_One::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchRepeatExprInvalid_Two {
pub second: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchRepeatExprInvalid_Two {
type Root = SwitchRepeatExprInvalid;
type ParentStack = (&'r SwitchRepeatExprInvalid, <SwitchRepeatExprInvalid as KStruct<'r, 's>>::ParentStack);

fn read<S: KStream>(
    &mut self,
    _io: &'s S,
    _root: Option<&'r Self::Root>,
    _parent: Option<TypedStack<Self::ParentStack>>
) -> KResult<()> {
    self.second = _io.read_bytes_full()?;
    Ok(())
}
}
impl<'r, 's: 'r> SwitchRepeatExprInvalid_Two {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchRepeatExprInvalid_Two::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
