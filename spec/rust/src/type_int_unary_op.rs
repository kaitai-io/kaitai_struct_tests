// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct TypeIntUnaryOp {
    pub valueS2: i16,
    pub valueS8: i64,
    pub unaryS2: Option<i32>,
    pub unaryS8: Option<i64>,
}

impl KaitaiStruct for TypeIntUnaryOp {
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
        self.valueS2 = self.stream.read_s2le()?;
        self.valueS8 = self.stream.read_s8le()?;
    }
}

impl TypeIntUnaryOp {
    fn unaryS2(&mut self) -> i32 {
        if let Some(x) = self.unaryS2 {
            return x;
        }

        self.unaryS2 = -(self.value_s2);
        return self.unaryS2;
    }
    fn unaryS8(&mut self) -> i64 {
        if let Some(x) = self.unaryS8 {
            return x;
        }

        self.unaryS8 = -(self.value_s8);
        return self.unaryS8;
    }
}
