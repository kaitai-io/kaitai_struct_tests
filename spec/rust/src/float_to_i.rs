// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct FloatToI {
    pub singleValue: f32,
    pub doubleValue: f64,
    pub float2I: Option<i32>,
    pub calcFloat1: Option<f64>,
    pub float4I: Option<i32>,
    pub calcFloat3: Option<f64>,
    pub calcFloat2: Option<f64>,
    pub float1I: Option<i32>,
    pub doubleI: Option<i32>,
    pub float3I: Option<i32>,
    pub singleI: Option<i32>,
    pub calcFloat4: Option<f64>,
}

impl KaitaiStruct for FloatToI {
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
    }
}

impl FloatToI {
    fn float2I(&mut self) -> i32 {
        if let Some(x) = self.float2I {
            return x;
        }

        self.float2I = self.calc_float2 as i32;
        return self.float2I;
    }
    fn calcFloat1(&mut self) -> f64 {
        if let Some(x) = self.calcFloat1 {
            return x;
        }

        self.calcFloat1 = 1.234;
        return self.calcFloat1;
    }
    fn float4I(&mut self) -> i32 {
        if let Some(x) = self.float4I {
            return x;
        }

        self.float4I = self.calc_float4 as i32;
        return self.float4I;
    }
    fn calcFloat3(&mut self) -> f64 {
        if let Some(x) = self.calcFloat3 {
            return x;
        }

        self.calcFloat3 = 1.9;
        return self.calcFloat3;
    }
    fn calcFloat2(&mut self) -> f64 {
        if let Some(x) = self.calcFloat2 {
            return x;
        }

        self.calcFloat2 = 1.5;
        return self.calcFloat2;
    }
    fn float1I(&mut self) -> i32 {
        if let Some(x) = self.float1I {
            return x;
        }

        self.float1I = self.calc_float1 as i32;
        return self.float1I;
    }
    fn doubleI(&mut self) -> i32 {
        if let Some(x) = self.doubleI {
            return x;
        }

        self.doubleI = self.double_value as i32;
        return self.doubleI;
    }
    fn float3I(&mut self) -> i32 {
        if let Some(x) = self.float3I {
            return x;
        }

        self.float3I = self.calc_float3 as i32;
        return self.float3I;
    }
    fn singleI(&mut self) -> i32 {
        if let Some(x) = self.singleI {
            return x;
        }

        self.singleI = self.single_value as i32;
        return self.singleI;
    }
    fn calcFloat4(&mut self) -> f64 {
        if let Some(x) = self.calcFloat4 {
            return x;
        }

        self.calcFloat4 = -2.7;
        return self.calcFloat4;
    }
}
