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
pub struct NavParentSwitch {
    pub category: u8,
    pub content: Box<NavParentSwitch__Element1>,
}

impl KaitaiStruct for NavParentSwitch {
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
        self.category = self.stream.read_u1()?;
        match self.category {
            1 => {
                self.content = Box::new(NavParentSwitch__Element1::new(self.stream, self, _root)?);
            },
        }
    }
}

impl NavParentSwitch {
}
#[derive(Default)]
pub struct NavParentSwitch__Element1 {
    pub foo: u8,
    pub subelement: Box<NavParentSwitch__Subelement1>,
}

impl KaitaiStruct for NavParentSwitch__Element1 {
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
        self.foo = self.stream.read_u1()?;
        self.subelement = Box::new(NavParentSwitch__Subelement1::new(self.stream, self, _root)?);
    }
}

impl NavParentSwitch__Element1 {
}
#[derive(Default)]
pub struct NavParentSwitch__Subelement1 {
    pub bar: u8,
}

impl KaitaiStruct for NavParentSwitch__Subelement1 {
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
        if self._parent.foo == 66 {
            self.bar = self.stream.read_u1()?;
        }
    }
}

impl NavParentSwitch__Subelement1 {
}
