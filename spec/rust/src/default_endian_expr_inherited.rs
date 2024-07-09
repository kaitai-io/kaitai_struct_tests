// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct DefaultEndianExprInherited {
    pub docs: Vec<Box<DefaultEndianExprInherited__Doc>>,
}

impl KaitaiStruct for DefaultEndianExprInherited {
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
            self.docs.push(Box::new(DefaultEndianExprInherited__Doc::new(self.stream, self, _root)?));
        }
    }
}

impl DefaultEndianExprInherited {
}
#[derive(Default)]
pub struct DefaultEndianExprInherited__Doc {
    pub indicator: Vec<u8>,
    pub main: Box<DefaultEndianExprInherited__Doc__MainObj>,
}

impl KaitaiStruct for DefaultEndianExprInherited__Doc {
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
        self.main = Box::new(DefaultEndianExprInherited__Doc__MainObj::new(self.stream, self, _root)?);
    }
}

impl DefaultEndianExprInherited__Doc {
}
#[derive(Default)]
pub struct DefaultEndianExprInherited__Doc__MainObj {
    pub insides: Box<DefaultEndianExprInherited__Doc__MainObj__SubObj>,
}

impl KaitaiStruct for DefaultEndianExprInherited__Doc__MainObj {
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
        self.insides = Box::new(DefaultEndianExprInherited__Doc__MainObj__SubObj::new(self.stream, self, _root)?);
    }
}

impl DefaultEndianExprInherited__Doc__MainObj {
}
#[derive(Default)]
pub struct DefaultEndianExprInherited__Doc__MainObj__SubObj {
    pub someInt: u32,
    pub more: Box<DefaultEndianExprInherited__Doc__MainObj__SubObj__SubsubObj>,
}

impl KaitaiStruct for DefaultEndianExprInherited__Doc__MainObj__SubObj {
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
        self.more = Box::new(DefaultEndianExprInherited__Doc__MainObj__SubObj__SubsubObj::new(self.stream, self, _root)?);
    }
}

impl DefaultEndianExprInherited__Doc__MainObj__SubObj {
}
#[derive(Default)]
pub struct DefaultEndianExprInherited__Doc__MainObj__SubObj__SubsubObj {
    pub someInt1: u16,
    pub someInt2: u16,
    pub someInst: Option<u32>,
}

impl KaitaiStruct for DefaultEndianExprInherited__Doc__MainObj__SubObj__SubsubObj {
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
        self.someInt1 = self.stream.read_u2()?;
        self.someInt2 = self.stream.read_u2()?;
    }
}

impl DefaultEndianExprInherited__Doc__MainObj__SubObj__SubsubObj {
    fn someInst(&mut self) -> u32 {
        if let Some(x) = self.someInst {
            return x;
        }

        let _pos = self.stream.pos();
        self.stream.seek(2);
        self.someInst = self.stream.read_u4()?;
        self.stream.seek(_pos);
        return self.someInst;
    }
}
