// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct SwitchElseOnly {
    pub opcode: i8,
    pub prim_byte: Option<SwitchElseOnly_PrimByte>,
    pub indicator: Vec<u8>,
    pub ut: Option<SwitchElseOnly_Ut>,
}
#[derive(Debug, PartialEq)]
pub enum SwitchElseOnly_PrimByte {
    S1(i8),
}
impl From<i8> for SwitchElseOnly_PrimByte {
    fn from(v: i8) -> Self {
        Self::S1(v)
    }
}

#[derive(Debug, PartialEq)]
pub enum SwitchElseOnly_Ut {
    SwitchElseOnly_Data(SwitchElseOnly_Data),
}
impl From<SwitchElseOnly_Data> for SwitchElseOnly_Ut {
    fn from(v: SwitchElseOnly_Data) -> Self {
        Self::SwitchElseOnly_Data(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for SwitchElseOnly {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.opcode = _io.read_s1()?;
        self.prim_byte = Some(_io.read_s1()?);
        self.indicator = _io.read_bytes(4 as usize)?.to_vec();
        self.ut = Some(Self::read_into::<S, SwitchElseOnly_Data>(_io, _root, _parent.push(self))?.into());
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchElseOnly {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchElseOnly::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchElseOnly_Data {
    pub value: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchElseOnly_Data {
    type Root = SwitchElseOnly;
    type ParentStack = (&'r SwitchElseOnly, <SwitchElseOnly as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.value = _io.read_bytes(4 as usize)?.to_vec();
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchElseOnly_Data {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchElseOnly_Data::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
