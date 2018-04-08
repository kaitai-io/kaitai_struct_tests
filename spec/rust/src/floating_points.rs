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

pub struct FloatingPoints {
    pub singleValuePlusInt: f64,
    pub singleValuePlusFloat: f64,
    pub doubleValuePlusFloat: f64,
    pub singleValue: f32,
    pub doubleValue: f64,
    pub singleValueBe: f32,
    pub doubleValueBe: f64,
    pub approximateValue: f32,
}

impl KaitaiStruct for FloatingPoints {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            singleValuePlusInt: f64,
            singleValuePlusFloat: f64,
            doubleValuePlusFloat: f64,
            singleValue: f32,
            doubleValue: f64,
            singleValueBe: f32,
            doubleValueBe: f64,
            approximateValue: f32,
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
        self.singleValueBe = stream.read_f4be()?;
        self.doubleValueBe = stream.read_f8be()?;
        self.approximateValue = stream.read_f4le()?;

        Ok(())
    }
    public function singleValuePlusInt() {
        if (self.singleValuePlusInt !== null)
            return self.singleValuePlusInt;
        self.singleValuePlusInt = ($this->singleValue() + 1);
        return self.singleValuePlusInt;
    }
    public function singleValuePlusFloat() {
        if (self.singleValuePlusFloat !== null)
            return self.singleValuePlusFloat;
        self.singleValuePlusFloat = ($this->singleValue() + 0.5);
        return self.singleValuePlusFloat;
    }
    public function doubleValuePlusFloat() {
        if (self.doubleValuePlusFloat !== null)
            return self.doubleValuePlusFloat;
        self.doubleValuePlusFloat = ($this->doubleValue() + 0.05);
        return self.doubleValuePlusFloat;
    }
}
