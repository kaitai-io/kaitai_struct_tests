// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct FloatingPoints {
    pub single_value: f32,
    pub double_value: f64,
    pub single_value_be: f32,
    pub double_value_be: f64,
    pub approximate_value: f32,
    pub single_value_plus_int: Option<f64>,
    pub single_value_plus_float: Option<f64>,
    pub double_value_plus_float: Option<f64>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for FloatingPoints {
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
        self.single_value_be = _io.read_f4be()?;
        self.double_value_be = _io.read_f8be()?;
        self.approximate_value = _io.read_f4le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> FloatingPoints {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = FloatingPoints::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn single_value_plus_int<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&f64> {
        if self.single_value_plus_int.is_some() {
            return Ok(self.single_value_plus_int.as_ref().unwrap());
        }
        self.single_value_plus_int = Some((self.single_value + 1) as f64);
        return Ok(self.single_value_plus_int.as_ref().unwrap());
    }
    fn single_value_plus_float<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&f64> {
        if self.single_value_plus_float.is_some() {
            return Ok(self.single_value_plus_float.as_ref().unwrap());
        }
        self.single_value_plus_float = Some((self.single_value + 0.5) as f64);
        return Ok(self.single_value_plus_float.as_ref().unwrap());
    }
    fn double_value_plus_float<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&f64> {
        if self.double_value_plus_float.is_some() {
            return Ok(self.double_value_plus_float.as_ref().unwrap());
        }
        self.double_value_plus_float = Some((self.double_value + 0.05) as f64);
        return Ok(self.double_value_plus_float.as_ref().unwrap());
    }
}
