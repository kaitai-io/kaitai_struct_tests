// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
// extraAttrForIO(RawIdentifier(InstanceIdentifier(mess_up)), NoRepeat)
// extraAttrForIO(RawIdentifier(InstanceIdentifier(mess_up)), NoRepeat)

#[derive(Default, Debug, PartialEq)]
pub struct IoLocalVar {
    pub skip: Vec<u8>,
    pub always_null: u8,
    pub followup: u8,
    pub raw_mess_up: Vec<u8>,
    pub mess_up: Option<IoLocalVar_MessUp>,
}
#[derive(Debug, PartialEq)]
pub enum IoLocalVar_MessUp {
    IoLocalVar_Dummy(IoLocalVar_Dummy),
    Bytes(Vec<u8>),
}
impl From<IoLocalVar_Dummy> for IoLocalVar_MessUp {
    fn from(v: IoLocalVar_Dummy) -> Self {
        Self::IoLocalVar_Dummy(v)
    }
}
impl From<Vec<u8>> for IoLocalVar_MessUp {
    fn from(v: Vec<u8>) -> Self {
        Self::Bytes(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for IoLocalVar {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.skip = _io.read_bytes(20 as usize)?.to_vec();
        {
            // condIfHeader(Compare(Attribute(Attribute(CastToType(Name(identifier(mess_up)),typeId(false,ArrayBuffer(dummy),false)),identifier(_io)),identifier(pos)),Lt,IntNum(0)))
            self.always_null = _io.read_u1()?;
        }
        self.followup = _io.read_u1()?;
        Ok(())
    }
}
impl<'r, 's: 'r> IoLocalVar {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = IoLocalVar::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn mess_up<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&IoLocalVar_MessUp> {
        if self.mess_up.is_some() {
            return Ok(self.mess_up.as_ref().unwrap());
        }
        // pushPos(// useIO(Attribute(Name(identifier(_root)),identifier(_io))))
        // seek(// useIO(Attribute(Name(identifier(_root)),identifier(_io))), IntNum(8))
        match 2 {
            1 => {
            }
            2 => {
            }
            // switchElseStart()
        }
        _ => panic!("unhandled value")
    }
    // popPos(// useIO(Attribute(Name(identifier(_root)),identifier(_io))))
    return Ok(self.mess_up.as_ref().unwrap());
}
}

#[derive(Default, Debug, PartialEq)]
pub struct IoLocalVar_Dummy {
}
impl<'r, 's: 'r> KStruct<'r, 's> for IoLocalVar_Dummy {
type Root = IoLocalVar;
type ParentStack = (&'r IoLocalVar, <IoLocalVar as KStruct<'r, 's>>::ParentStack);

fn read<S: KStream>(
    &mut self,
    _io: &'s S,
    _root: Option<&'r Self::Root>,
    _parent: Option<TypedStack<Self::ParentStack>>
) -> KResult<()> {
    Ok(())
}
}
impl<'r, 's: 'r> IoLocalVar_Dummy {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = IoLocalVar_Dummy::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
