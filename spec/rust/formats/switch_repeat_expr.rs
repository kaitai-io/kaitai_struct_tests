// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
// extraAttrForIO(RawIdentifier(NamedIdentifier(body)), RepeatExpr(IntNum(1)))
// extraAttrForIO(RawIdentifier(NamedIdentifier(body)), RepeatExpr(IntNum(1)))

#[derive(Default, Debug, PartialEq)]
pub struct SwitchRepeatExpr {
    pub code: u8,
    pub size: u32,
    pub body: Vec<SwitchRepeatExpr_Body>,
    pub raw_body: Vec<Vec<u8>>,
}
#[derive(Debug, PartialEq)]
pub enum SwitchRepeatExpr_Body {
    SwitchRepeatExpr_One(SwitchRepeatExpr_One),
    SwitchRepeatExpr_Two(SwitchRepeatExpr_Two),
    Bytes(Vec<u8>),
}
impl From<SwitchRepeatExpr_One> for SwitchRepeatExpr_Body {
    fn from(v: SwitchRepeatExpr_One) -> Self {
        Self::SwitchRepeatExpr_One(v)
    }
}
impl From<SwitchRepeatExpr_Two> for SwitchRepeatExpr_Body {
    fn from(v: SwitchRepeatExpr_Two) -> Self {
        Self::SwitchRepeatExpr_Two(v)
    }
}
impl From<Vec<u8>> for SwitchRepeatExpr_Body {
    fn from(v: Vec<u8>) -> Self {
        Self::Bytes(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for SwitchRepeatExpr {
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
            // condRepeatExprHeader(NamedIdentifier(body), _io, SwitchType(Name(identifier(code)),Map(IntNum(17) -> UserTypeFromBytes(List(one),None,List(),BytesLimitType(Name(identifier(size)),None,false,None,None),None), IntNum(34) -> UserTypeFromBytes(List(two),None,List(),BytesLimitType(Name(identifier(size)),None,false,None,None),None), Name(identifier(_)) -> BytesLimitType(Name(identifier(size)),None,false,None,None)),true), IntNum(1))
            match self.code {
                17 => {
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
impl<'r, 's: 'r> SwitchRepeatExpr {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchRepeatExpr::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchRepeatExpr_One {
pub first: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchRepeatExpr_One {
type Root = SwitchRepeatExpr;
type ParentStack = (&'r SwitchRepeatExpr, <SwitchRepeatExpr as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> SwitchRepeatExpr_One {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchRepeatExpr_One::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchRepeatExpr_Two {
pub second: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchRepeatExpr_Two {
type Root = SwitchRepeatExpr;
type ParentStack = (&'r SwitchRepeatExpr, <SwitchRepeatExpr as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> SwitchRepeatExpr_Two {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchRepeatExpr_Two::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
