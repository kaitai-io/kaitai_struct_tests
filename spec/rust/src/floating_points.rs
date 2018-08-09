// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::io::Cursor;
use std::vec::Vec;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct FloatingPoints {
    pub singleValue: f32,
    pub doubleValue: f64,
    pub singleValueBe: f32,
    pub doubleValueBe: f64,
    pub approximateValue: f32,
    pub singleValuePlusInt: Option<f64>,
    pub singleValuePlusFloat: Option<f64>,
    pub doubleValuePlusFloat: Option<f64>,
}

impl KaitaiStruct for FloatingPoints {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s: Self = Default::default();

        s.stream = stream;
        s.read(stream, _parent, _root)?;

        Ok(s)
    }


    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S,
                             _parent: &Option<Box<KaitaiStruct>>,
                             _root: &Option<Box<KaitaiStruct>>)
                             -> Result<()>
        where Self: Sized {
        self.singleValue = self.stream.read_f4le()?;
        self.doubleValue = self.stream.read_f8le()?;
        self.singleValueBe = self.stream.read_f4be()?;
        self.doubleValueBe = self.stream.read_f8be()?;
        self.approximateValue = self.stream.read_f4le()?;
    }
}

impl FloatingPoints {
    fn singleValuePlusInt(&mut self) -> f64 {
        if let Some(x) = self.singleValuePlusInt {
            return x;
        }

        self.singleValuePlusInt = (self.single_value + 1);
        return self.singleValuePlusInt;
    }
    fn singleValuePlusFloat(&mut self) -> f64 {
        if let Some(x) = self.singleValuePlusFloat {
            return x;
        }

        self.singleValuePlusFloat = (self.single_value + 0.5);
        return self.singleValuePlusFloat;
    }
    fn doubleValuePlusFloat(&mut self) -> f64 {
        if let Some(x) = self.doubleValuePlusFloat {
            return x;
        }

        self.doubleValuePlusFloat = (self.double_value + 0.05);
        return self.doubleValuePlusFloat;
    }
}
