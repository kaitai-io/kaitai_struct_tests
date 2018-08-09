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
pub struct NestedTypes2 {
    pub one: Box<NestedTypes2__SubtypeA>,
    pub two: Box<NestedTypes2__SubtypeB>,
}

impl KaitaiStruct for NestedTypes2 {
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
        self.one = Box::new(NestedTypes2__SubtypeA::new(self.stream, self, _root)?);
        self.two = Box::new(NestedTypes2__SubtypeB::new(self.stream, self, _root)?);
    }
}

impl NestedTypes2 {
}
#[derive(Default)]
pub struct NestedTypes2__SubtypeA {
    pub typedAtRoot: Box<NestedTypes2__SubtypeB>,
    pub typedHere1: Box<NestedTypes2__SubtypeA__SubtypeC>,
    pub typedHere2: Box<NestedTypes2__SubtypeA__SubtypeCc>,
}

impl KaitaiStruct for NestedTypes2__SubtypeA {
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
        self.typedAtRoot = Box::new(NestedTypes2__SubtypeB::new(self.stream, self, _root)?);
        self.typedHere1 = Box::new(NestedTypes2__SubtypeA__SubtypeC::new(self.stream, self, _root)?);
        self.typedHere2 = Box::new(NestedTypes2__SubtypeA__SubtypeCc::new(self.stream, self, _root)?);
    }
}

impl NestedTypes2__SubtypeA {
}
#[derive(Default)]
pub struct NestedTypes2__SubtypeA__SubtypeC {
    pub valueC: i8,
    pub typedHere: Box<NestedTypes2__SubtypeA__SubtypeC__SubtypeD>,
    pub typedParent: Box<NestedTypes2__SubtypeA__SubtypeCc>,
    pub typedRoot: Box<NestedTypes2__SubtypeB>,
}

impl KaitaiStruct for NestedTypes2__SubtypeA__SubtypeC {
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
        self.typedHere = Box::new(NestedTypes2__SubtypeA__SubtypeC__SubtypeD::new(self.stream, self, _root)?);
        self.typedParent = Box::new(NestedTypes2__SubtypeA__SubtypeCc::new(self.stream, self, _root)?);
        self.typedRoot = Box::new(NestedTypes2__SubtypeB::new(self.stream, self, _root)?);
    }
}

impl NestedTypes2__SubtypeA__SubtypeC {
}
#[derive(Default)]
pub struct NestedTypes2__SubtypeA__SubtypeC__SubtypeD {
    pub valueD: i8,
}

impl KaitaiStruct for NestedTypes2__SubtypeA__SubtypeC__SubtypeD {
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
        self.valueD = self.stream.read_s1()?;
    }
}

impl NestedTypes2__SubtypeA__SubtypeC__SubtypeD {
}
#[derive(Default)]
pub struct NestedTypes2__SubtypeA__SubtypeCc {
    pub valueCc: i8,
}

impl KaitaiStruct for NestedTypes2__SubtypeA__SubtypeCc {
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
        self.valueCc = self.stream.read_s1()?;
    }
}

impl NestedTypes2__SubtypeA__SubtypeCc {
}
#[derive(Default)]
pub struct NestedTypes2__SubtypeB {
    pub valueB: i8,
}

impl KaitaiStruct for NestedTypes2__SubtypeB {
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

impl NestedTypes2__SubtypeB {
}
