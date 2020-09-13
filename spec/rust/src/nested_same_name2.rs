// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct NestedSameName2 {
    pub version: u32,
    pub mainData: Box<NestedSameName2__Main>,
    pub dummy: Box<NestedSameName2__DummyObj>,
}

impl KaitaiStruct for NestedSameName2 {
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
        self.version = self.stream.read_u4le()?;
        self.mainData = Box::new(NestedSameName2__Main::new(self.stream, self, _root)?);
        self.dummy = Box::new(NestedSameName2__DummyObj::new(self.stream, self, _root)?);
    }
}

impl NestedSameName2 {
}
#[derive(Default)]
pub struct NestedSameName2__Main {
    pub mainSize: i32,
    pub foo: Box<NestedSameName2__Main__FooObj>,
}

impl KaitaiStruct for NestedSameName2__Main {
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
        self.mainSize = self.stream.read_s4le()?;
        self.foo = Box::new(NestedSameName2__Main__FooObj::new(self.stream, self, _root)?);
    }
}

impl NestedSameName2__Main {
}
#[derive(Default)]
pub struct NestedSameName2__Main__FooObj {
    pub data1: Vec<u8>,
}

impl KaitaiStruct for NestedSameName2__Main__FooObj {
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
        self.data1 = self.stream.read_bytes((self._parent.main_size * 2))?;
    }
}

impl NestedSameName2__Main__FooObj {
}
#[derive(Default)]
pub struct NestedSameName2__DummyObj {
    pub dummySize: i32,
    pub foo: Box<NestedSameName2__DummyObj__FooObj>,
}

impl KaitaiStruct for NestedSameName2__DummyObj {
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
        self.dummySize = self.stream.read_s4le()?;
        self.foo = Box::new(NestedSameName2__DummyObj__FooObj::new(self.stream, self, _root)?);
    }
}

impl NestedSameName2__DummyObj {
}
#[derive(Default)]
pub struct NestedSameName2__DummyObj__FooObj {
    pub data2: Vec<u8>,
}

impl KaitaiStruct for NestedSameName2__DummyObj__FooObj {
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
        self.data2 = self.stream.read_bytes((self._parent.dummy_size * 2))?;
    }
}

impl NestedSameName2__DummyObj__FooObj {
}
