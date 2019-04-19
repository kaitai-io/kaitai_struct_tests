// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct NavParentOverride {
    pub childSize: u8,
    pub child1: Box<NavParentOverride__Child>,
    pub mediator2: Box<NavParentOverride__Mediator>,
}

impl KaitaiStruct for NavParentOverride {
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
        self.childSize = self.stream.read_u1()?;
        self.child1 = Box::new(NavParentOverride__Child::new(self.stream, self, _root)?);
        self.mediator2 = Box::new(NavParentOverride__Mediator::new(self.stream, self, _root)?);
    }
}

impl NavParentOverride {
}
#[derive(Default)]
pub struct NavParentOverride__Mediator {
    pub child2: Box<NavParentOverride__Child>,
}

impl KaitaiStruct for NavParentOverride__Mediator {
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
        self.child2 = Box::new(NavParentOverride__Child::new(self.stream, self, _root)?);
    }
}

impl NavParentOverride__Mediator {
}
#[derive(Default)]
pub struct NavParentOverride__Child {
    pub data: Vec<u8>,
}

impl KaitaiStruct for NavParentOverride__Child {
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
        self.data = self.stream.read_bytes(self._parent.child_size)?;
    }
}

impl NavParentOverride__Child {
}
