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
pub struct DefaultEndianMod {
    pub main: Box<DefaultEndianMod__MainObj>,
}

impl KaitaiStruct for DefaultEndianMod {
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
        self.main = Box::new(DefaultEndianMod__MainObj::new(self.stream, self, _root)?);
    }
}

impl DefaultEndianMod {
}
#[derive(Default)]
pub struct DefaultEndianMod__MainObj {
    pub one: i32,
    pub nest: Box<DefaultEndianMod__MainObj__Subnest>,
    pub nestBe: Box<DefaultEndianMod__MainObj__SubnestBe>,
}

impl KaitaiStruct for DefaultEndianMod__MainObj {
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
        self.one = self.stream.read_s4le()?;
        self.nest = Box::new(DefaultEndianMod__MainObj__Subnest::new(self.stream, self, _root)?);
        self.nestBe = Box::new(DefaultEndianMod__MainObj__SubnestBe::new(self.stream, self, _root)?);
    }
}

impl DefaultEndianMod__MainObj {
}
#[derive(Default)]
pub struct DefaultEndianMod__MainObj__Subnest {
    pub two: i32,
}

impl KaitaiStruct for DefaultEndianMod__MainObj__Subnest {
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
        self.two = self.stream.read_s4le()?;
    }
}

impl DefaultEndianMod__MainObj__Subnest {
}
#[derive(Default)]
pub struct DefaultEndianMod__MainObj__SubnestBe {
    pub two: i32,
}

impl KaitaiStruct for DefaultEndianMod__MainObj__SubnestBe {
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
        self.two = self.stream.read_s4be()?;
    }
}

impl DefaultEndianMod__MainObj__SubnestBe {
}
