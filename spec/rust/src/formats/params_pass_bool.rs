// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ParamsPassBool {
    pub s_false: bool,
    pub s_true: bool,
    pub seq_b1: Option<ParamsPassBool_ParamTypeB1>,
    pub seq_bool: Option<ParamsPassBool_ParamTypeBool>,
    pub literal_b1: Option<ParamsPassBool_ParamTypeB1>,
    pub literal_bool: Option<ParamsPassBool_ParamTypeBool>,
    pub inst_b1: Option<ParamsPassBool_ParamTypeB1>,
    pub inst_bool: Option<ParamsPassBool_ParamTypeBool>,
    pub v_false: Option<bool>,
    pub v_true: Option<bool>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ParamsPassBool {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.s_false = _io.read_bits_int(1)? != 0;
        self.s_true = _io.read_bits_int(1)? != 0;
        _io.align_to_byte()?;
        self.seq_b1 = Some(Self::read_into::<BytesReader, ParamsPassBool_ParamTypeB1>(Self::read_into::<S, ParamsPassBool_ParamTypeB1>(self.s_true, _io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.seq_bool = Some(Self::read_into::<BytesReader, ParamsPassBool_ParamTypeBool>(Self::read_into::<S, ParamsPassBool_ParamTypeBool>(self.s_false, _io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.literal_b1 = Some(Self::read_into::<BytesReader, ParamsPassBool_ParamTypeB1>(Self::read_into::<S, ParamsPassBool_ParamTypeB1>(false, _io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.literal_bool = Some(Self::read_into::<BytesReader, ParamsPassBool_ParamTypeBool>(Self::read_into::<S, ParamsPassBool_ParamTypeBool>(true, _io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.inst_b1 = Some(Self::read_into::<BytesReader, ParamsPassBool_ParamTypeB1>(Self::read_into::<S, ParamsPassBool_ParamTypeB1>(self.v_true(_io, _root, _parent)?, _io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.inst_bool = Some(Self::read_into::<BytesReader, ParamsPassBool_ParamTypeBool>(Self::read_into::<S, ParamsPassBool_ParamTypeBool>(self.v_false(_io, _root, _parent)?, _io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> ParamsPassBool {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ParamsPassBool::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn v_false<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.v_false.is_some() {
            return Ok(self.v_false.as_ref().unwrap());
        }
        self.v_false = Some(false as bool);
        return Ok(self.v_false.as_ref().unwrap());
    }
    fn v_true<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.v_true.is_some() {
            return Ok(self.v_true.as_ref().unwrap());
        }
        self.v_true = Some(true as bool);
        return Ok(self.v_true.as_ref().unwrap());
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct ParamsPassBool_ParamTypeB1 {
    pub arg: bool,
    pub foo: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ParamsPassBool_ParamTypeB1 {
    type Root = ParamsPassBool;
    type ParentStack = (&'r ParamsPassBool, <ParamsPassBool as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.foo = _io.read_bytes(if self.arg { 1 } else { 2} as usize)?.to_vec();
        Ok(())
    }
}
impl<'r, 's: 'r> ParamsPassBool_ParamTypeB1 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ParamsPassBool_ParamTypeB1::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct ParamsPassBool_ParamTypeBool {
    pub arg: bool,
    pub foo: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ParamsPassBool_ParamTypeBool {
    type Root = ParamsPassBool;
    type ParentStack = (&'r ParamsPassBool, <ParamsPassBool as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.foo = _io.read_bytes(if self.arg { 1 } else { 2} as usize)?.to_vec();
        Ok(())
    }
}
impl<'r, 's: 'r> ParamsPassBool_ParamTypeBool {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ParamsPassBool_ParamTypeBool::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
