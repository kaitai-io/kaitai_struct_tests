// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct DefaultEndianExprIsBe {
    pub docs: Vec<Box<DefaultEndianExprIsBe__Doc>>,
}

impl KaitaiStruct for DefaultEndianExprIsBe {
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
            self.docs.push(Box::new(DefaultEndianExprIsBe__Doc::new(self.stream, self, _root)?));
        }
    }
}

impl DefaultEndianExprIsBe {
}
#[derive(Default)]
pub struct DefaultEndianExprIsBe__Doc {
    pub indicator: Vec<u8>,
    pub main: Box<DefaultEndianExprIsBe__Doc__MainObj>,
}

impl KaitaiStruct for DefaultEndianExprIsBe__Doc {
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
        self.main = Box::new(DefaultEndianExprIsBe__Doc__MainObj::new(self.stream, self, _root)?);
    }
}

impl DefaultEndianExprIsBe__Doc {
}
#[derive(Default)]
pub struct DefaultEndianExprIsBe__Doc__MainObj {
    pub someInt: u32,
    pub someIntBe: u16,
    pub someIntLe: u16,
    pub instInt: Option<u32>,
    pub instSub: Option<Box<DefaultEndianExprIsBe__Doc__MainObj__SubMainObj>>,
}

impl KaitaiStruct for DefaultEndianExprIsBe__Doc__MainObj {
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

impl DefaultEndianExprIsBe__Doc__MainObj {
    fn instInt(&mut self) -> u32 {
        if let Some(x) = self.instInt {
            return x;
        }

        let _pos = self.stream.pos();
        self.stream.seek(2);
        self.instInt = self.stream.read_u4()?;
        self.stream.seek(_pos);
        return self.instInt;
    }
    fn instSub(&mut self) -> Box<DefaultEndianExprIsBe__Doc__MainObj__SubMainObj> {
        if let Some(x) = self.instSub {
            return x;
        }

        let _pos = self.stream.pos();
        self.stream.seek(2);
        self.instSub = Box::new(DefaultEndianExprIsBe__Doc__MainObj__SubMainObj::new(self.stream, self, _root)?);
        self.stream.seek(_pos);
        return self.instSub;
    }
}
#[derive(Default)]
pub struct DefaultEndianExprIsBe__Doc__MainObj__SubMainObj {
    pub foo: u32,
}

impl KaitaiStruct for DefaultEndianExprIsBe__Doc__MainObj__SubMainObj {
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
        self.foo = self.stream.read_u4()?;
    }
}

impl DefaultEndianExprIsBe__Doc__MainObj__SubMainObj {
}
