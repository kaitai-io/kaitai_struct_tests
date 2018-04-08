// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

use std::{
    option::Option,
    boxed::Box,
    io::Result
};

use kaitai_struct::{
    KaitaiStream,
    KaitaiStruct
};

pub struct FloatToI {
    pub float2I: i32,
    pub calcFloat1: f64,
    pub float4I: i32,
    pub calcFloat3: f64,
    pub calcFloat2: f64,
    pub float1I: i32,
    pub doubleI: i32,
    pub float3I: i32,
    pub singleI: i32,
    pub calcFloat4: f64,
    pub singleValue: f32,
    pub doubleValue: f64,
}

impl KaitaiStruct for FloatToI {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            float2I: i32,
            calcFloat1: f64,
            float4I: i32,
            calcFloat3: f64,
            calcFloat2: f64,
            float1I: i32,
            doubleI: i32,
            float3I: i32,
            singleI: i32,
            calcFloat4: f64,
            singleValue: f32,
            doubleValue: f64,
        };

        s.read(stream, _parent, _root)?;

        Ok(s)
    }

    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S,
                             _parent: &Option<Box<KaitaiStruct>>,
                             _root: &Option<Box<KaitaiStruct>>)
                             -> Result<()>
        where Self: Sized {
        self.singleValue = stream.read_f4le()?;
        self.doubleValue = stream.read_f8le()?;

        Ok(())
    }
    public function float2I() {
        if (self.float2I !== null)
            return self.float2I;
        self.float2I = intval($this->calcFloat2());
        return self.float2I;
    }
    public function calcFloat1() {
        if (self.calcFloat1 !== null)
            return self.calcFloat1;
        self.calcFloat1 = 1.234;
        return self.calcFloat1;
    }
    public function float4I() {
        if (self.float4I !== null)
            return self.float4I;
        self.float4I = intval($this->calcFloat4());
        return self.float4I;
    }
    public function calcFloat3() {
        if (self.calcFloat3 !== null)
            return self.calcFloat3;
        self.calcFloat3 = 1.9;
        return self.calcFloat3;
    }
    public function calcFloat2() {
        if (self.calcFloat2 !== null)
            return self.calcFloat2;
        self.calcFloat2 = 1.5;
        return self.calcFloat2;
    }
    public function float1I() {
        if (self.float1I !== null)
            return self.float1I;
        self.float1I = intval($this->calcFloat1());
        return self.float1I;
    }
    public function doubleI() {
        if (self.doubleI !== null)
            return self.doubleI;
        self.doubleI = intval($this->doubleValue());
        return self.doubleI;
    }
    public function float3I() {
        if (self.float3I !== null)
            return self.float3I;
        self.float3I = intval($this->calcFloat3());
        return self.float3I;
    }
    public function singleI() {
        if (self.singleI !== null)
            return self.singleI;
        self.singleI = intval($this->singleValue());
        return self.singleI;
    }
    public function calcFloat4() {
        if (self.calcFloat4 !== null)
            return self.calcFloat4;
        self.calcFloat4 = -2.7;
        return self.calcFloat4;
    }
}
