// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct DefaultEndianExprIsLe {
    pub docs: Vec<Box<DefaultEndianExprIsLe__Doc>>,
}

impl KaitaiStruct for DefaultEndianExprIsLe {
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
        self.docs = [];
        while !self.stream.isEof() {
            self.docs.push(Box::new(DefaultEndianExprIsLe__Doc::new(self.stream, self, _root)?));
        }
    }
}

impl DefaultEndianExprIsLe {
}
#[derive(Default)]
pub struct DefaultEndianExprIsLe__Doc {
    pub indicator: Vec<u8>,
    pub main: Box<DefaultEndianExprIsLe__Doc__MainObj>,
}

impl KaitaiStruct for DefaultEndianExprIsLe__Doc {
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
        self.indicator = self.stream.read_bytes(2)?;
        self.main = Box::new(DefaultEndianExprIsLe__Doc__MainObj::new(self.stream, self, _root)?);
    }
}

impl DefaultEndianExprIsLe__Doc {
}
#[derive(Default)]
pub struct DefaultEndianExprIsLe__Doc__MainObj {
    pub someInt: u32,
    pub someIntBe: u16,
    pub someIntLe: u16,
}

impl KaitaiStruct for DefaultEndianExprIsLe__Doc__MainObj {
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
        self.someInt = self.stream.read_u4()?;
        self.someIntBe = self.stream.read_u2be()?;
        self.someIntLe = self.stream.read_u2le()?;
    }
}

impl DefaultEndianExprIsLe__Doc__MainObj {
}
