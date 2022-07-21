// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct FloatToI {
    pub single_value: f32,
    pub double_value: f64,
    pub float2_i: Option<i32>,
    pub calc_float1: Option<f64>,
    pub float4_i: Option<i32>,
    pub calc_float3: Option<f64>,
    pub calc_float2: Option<f64>,
    pub float1_i: Option<i32>,
    pub double_i: Option<i32>,
    pub float3_i: Option<i32>,
    pub single_i: Option<i32>,
    pub calc_float4: Option<f64>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for FloatToI {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.single_value = _io.read_f4le()?;
        self.double_value = _io.read_f8le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> FloatToI {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = FloatToI::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn float2_i<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.float2_i.is_some() {
            return Ok(self.float2_i.as_ref().unwrap());
        }
        self.float2_i = Some(self.calc_float2(_io, _root, _parent)? as i32 as i32);
        return Ok(self.float2_i.as_ref().unwrap());
    }
    fn calc_float1<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&f64> {
        if self.calc_float1.is_some() {
            return Ok(self.calc_float1.as_ref().unwrap());
        }
        self.calc_float1 = Some(1.234 as f64);
        return Ok(self.calc_float1.as_ref().unwrap());
    }
    fn float4_i<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.float4_i.is_some() {
            return Ok(self.float4_i.as_ref().unwrap());
        }
        self.float4_i = Some(self.calc_float4(_io, _root, _parent)? as i32 as i32);
        return Ok(self.float4_i.as_ref().unwrap());
    }
    fn calc_float3<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&f64> {
        if self.calc_float3.is_some() {
            return Ok(self.calc_float3.as_ref().unwrap());
        }
        self.calc_float3 = Some(1.9 as f64);
        return Ok(self.calc_float3.as_ref().unwrap());
    }
    fn calc_float2<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&f64> {
        if self.calc_float2.is_some() {
            return Ok(self.calc_float2.as_ref().unwrap());
        }
        self.calc_float2 = Some(1.5 as f64);
        return Ok(self.calc_float2.as_ref().unwrap());
    }
    fn float1_i<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.float1_i.is_some() {
            return Ok(self.float1_i.as_ref().unwrap());
        }
        self.float1_i = Some(self.calc_float1(_io, _root, _parent)? as i32 as i32);
        return Ok(self.float1_i.as_ref().unwrap());
    }
    fn double_i<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.double_i.is_some() {
            return Ok(self.double_i.as_ref().unwrap());
        }
        self.double_i = Some(self.double_value as i32 as i32);
        return Ok(self.double_i.as_ref().unwrap());
    }
    fn float3_i<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.float3_i.is_some() {
            return Ok(self.float3_i.as_ref().unwrap());
        }
        self.float3_i = Some(self.calc_float3(_io, _root, _parent)? as i32 as i32);
        return Ok(self.float3_i.as_ref().unwrap());
    }
    fn single_i<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.single_i.is_some() {
            return Ok(self.single_i.as_ref().unwrap());
        }
        self.single_i = Some(self.single_value as i32 as i32);
        return Ok(self.single_i.as_ref().unwrap());
    }
    fn calc_float4<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&f64> {
        if self.calc_float4.is_some() {
            return Ok(self.calc_float4.as_ref().unwrap());
        }
        self.calc_float4 = Some(-2.7 as f64);
        return Ok(self.calc_float4.as_ref().unwrap());
    }
}
