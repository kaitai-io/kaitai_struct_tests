// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;
use std::vec::Vec;

#[derive(Default)]
pub struct RepeatUntilComplex {
    pub first: Vec<Box<RepeatUntilComplex__TypeU1>>,
    pub second: Vec<Box<RepeatUntilComplex__TypeU2>>,
    pub third: Vec<u8>,
}

impl KaitaiStruct for RepeatUntilComplex {
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
        self.first = vec!();
        while {
            let tmpa = Box::new(RepeatUntilComplex__TypeU1::new(self.stream, self, _root)?);
            self.first.append(Box::new(RepeatUntilComplex__TypeU1::new(self.stream, self, _root)?));
            !(tmpa.count == 0)
        } { }
        self.second = vec!();
        while {
            let tmpa = Box::new(RepeatUntilComplex__TypeU2::new(self.stream, self, _root)?);
            self.second.append(Box::new(RepeatUntilComplex__TypeU2::new(self.stream, self, _root)?));
            !(tmpa.count == 0)
        } { }
        self.third = vec!();
        while {
            let tmpa = self.stream.read_u1()?;
            self.third.append(self.stream.read_u1()?);
            !(tmpa == 0)
        } { }
    }
}

impl RepeatUntilComplex {
}
#[derive(Default)]
pub struct RepeatUntilComplex__TypeU1 {
    pub count: u8,
    pub values: Vec<u8>,
}

impl KaitaiStruct for RepeatUntilComplex__TypeU1 {
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
        self.count = self.stream.read_u1()?;
        self.values = vec!();
        for i in 0..self.count {
            self.values.push(self.stream.read_u1()?);
        }
    }
}

impl RepeatUntilComplex__TypeU1 {
}
#[derive(Default)]
pub struct RepeatUntilComplex__TypeU2 {
    pub count: u16,
    pub values: Vec<u16>,
}

impl KaitaiStruct for RepeatUntilComplex__TypeU2 {
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
        self.count = self.stream.read_u2le()?;
        self.values = vec!();
        for i in 0..self.count {
            self.values.push(self.stream.read_u2le()?);
        }
    }
}

impl RepeatUntilComplex__TypeU2 {
}
