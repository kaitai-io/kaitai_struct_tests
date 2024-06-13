// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct NavParentFalse {
    pub childSize: u8,
    pub elementA: Box<NavParentFalse__ParentA>,
    pub elementB: Box<NavParentFalse__ParentB>,
}

impl KaitaiStruct for NavParentFalse {
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
        self.elementA = Box::new(NavParentFalse__ParentA::new(self.stream, self, _root)?);
        self.elementB = Box::new(NavParentFalse__ParentB::new(self.stream, self, _root)?);
    }
}

impl NavParentFalse {
}
#[derive(Default)]
pub struct NavParentFalse__ParentA {
    pub foo: Box<NavParentFalse__Child>,
    pub bar: Box<NavParentFalse__ParentB>,
}

impl KaitaiStruct for NavParentFalse__ParentA {
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
        self.foo = Box::new(NavParentFalse__Child::new(self.stream, self, _root)?);
        self.bar = Box::new(NavParentFalse__ParentB::new(self.stream, self, _root)?);
    }
}

impl NavParentFalse__ParentA {
}
#[derive(Default)]
pub struct NavParentFalse__ParentB {
    pub foo: Box<NavParentFalse__Child>,
}

impl KaitaiStruct for NavParentFalse__ParentB {
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
        self.foo = Box::new(NavParentFalse__Child::new(self.stream, self, _root)?);
    }
}

impl NavParentFalse__ParentB {
}
#[derive(Default)]
pub struct NavParentFalse__Child {
    pub code: u8,
    pub more: Vec<u8>,
}

impl KaitaiStruct for NavParentFalse__Child {
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
        self.code = self.stream.read_u1()?;
        if self.code == 73 {
            self.more = self.stream.read_bytes(self._parent._parent.child_size)?;
        }
    }
}

impl NavParentFalse__Child {
}
