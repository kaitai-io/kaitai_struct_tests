// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct NestedTypes {
    pub one: Box<NestedTypes__SubtypeA>,
    pub two: Box<NestedTypes__SubtypeB>,
}

impl KaitaiStruct for NestedTypes {
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
        self.one = Box::new(NestedTypes__SubtypeA::new(self.stream, self, _root)?);
        self.two = Box::new(NestedTypes__SubtypeB::new(self.stream, self, _root)?);
    }
}

impl NestedTypes {
}
#[derive(Default)]
pub struct NestedTypes__SubtypeA {
    pub typedAtRoot: Box<NestedTypes__SubtypeB>,
    pub typedHere: Box<NestedTypes__SubtypeA__SubtypeC>,
}

impl KaitaiStruct for NestedTypes__SubtypeA {
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
        self.typedAtRoot = Box::new(NestedTypes__SubtypeB::new(self.stream, self, _root)?);
        self.typedHere = Box::new(NestedTypes__SubtypeA__SubtypeC::new(self.stream, self, _root)?);
    }
}

impl NestedTypes__SubtypeA {
}
#[derive(Default)]
pub struct NestedTypes__SubtypeA__SubtypeC {
    pub valueC: i8,
}

impl KaitaiStruct for NestedTypes__SubtypeA__SubtypeC {
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
        self.valueC = self.stream.read_s1()?;
    }
}

impl NestedTypes__SubtypeA__SubtypeC {
}
#[derive(Default)]
pub struct NestedTypes__SubtypeB {
    pub valueB: i8,
}

impl KaitaiStruct for NestedTypes__SubtypeB {
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
        self.valueB = self.stream.read_s1()?;
    }
}

impl NestedTypes__SubtypeB {
}
