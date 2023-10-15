// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct ExprMod {
    pub intU: u32,
    pub intS: i32,
    pub modPosConst: Option<i32>,
    pub modNegConst: Option<i32>,
    pub modPosSeq: Option<i32>,
    pub modNegSeq: Option<i32>,
}

impl KaitaiStruct for ExprMod {
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
        self.intU = self.stream.read_u4le()?;
        self.intS = self.stream.read_s4le()?;
    }
}

impl ExprMod {
    fn modPosConst(&mut self) -> i32 {
        if let Some(x) = self.modPosConst {
            return x;
        }

        self.modPosConst = 9837 % 13;
        return self.modPosConst;
    }
    fn modNegConst(&mut self) -> i32 {
        if let Some(x) = self.modNegConst {
            return x;
        }

        self.modNegConst = -9837 % 13;
        return self.modNegConst;
    }
    fn modPosSeq(&mut self) -> i32 {
        if let Some(x) = self.modPosSeq {
            return x;
        }

        self.modPosSeq = self.int_u % 13;
        return self.modPosSeq;
    }
    fn modNegSeq(&mut self) -> i32 {
        if let Some(x) = self.modNegSeq {
            return x;
        }

        self.modNegSeq = self.int_s % 13;
        return self.modNegSeq;
    }
}
