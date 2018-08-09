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
pub struct NestedTypes3 {
    pub aCc: Box<NestedTypes3__SubtypeA__SubtypeCc>,
    pub aCD: Box<NestedTypes3__SubtypeA__SubtypeC__SubtypeD>,
    pub b: Box<NestedTypes3__SubtypeB>,
}

impl KaitaiStruct for NestedTypes3 {
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
        self.aCc = Box::new(NestedTypes3__SubtypeA__SubtypeCc::new(self.stream, self, _root)?);
        self.aCD = Box::new(NestedTypes3__SubtypeA__SubtypeC__SubtypeD::new(self.stream, self, _root)?);
        self.b = Box::new(NestedTypes3__SubtypeB::new(self.stream, self, _root)?);
    }
}

impl NestedTypes3 {
}
#[derive(Default)]
pub struct NestedTypes3__SubtypeA {
}

impl KaitaiStruct for NestedTypes3__SubtypeA {
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
    }
}

impl NestedTypes3__SubtypeA {
}
#[derive(Default)]
pub struct NestedTypes3__SubtypeA__SubtypeC {
}

impl KaitaiStruct for NestedTypes3__SubtypeA__SubtypeC {
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
    }
}

impl NestedTypes3__SubtypeA__SubtypeC {
}
#[derive(Default)]
pub struct NestedTypes3__SubtypeA__SubtypeC__SubtypeD {
    pub valueD: i8,
}

impl KaitaiStruct for NestedTypes3__SubtypeA__SubtypeC__SubtypeD {
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

impl NestedTypes3__SubtypeA__SubtypeC__SubtypeD {
}
#[derive(Default)]
pub struct NestedTypes3__SubtypeA__SubtypeCc {
    pub valueCc: i8,
}

impl KaitaiStruct for NestedTypes3__SubtypeA__SubtypeCc {
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

impl NestedTypes3__SubtypeA__SubtypeCc {
}
#[derive(Default)]
pub struct NestedTypes3__SubtypeB {
    pub valueB: i8,
    pub aCc: Box<NestedTypes3__SubtypeA__SubtypeCc>,
    pub aCD: Box<NestedTypes3__SubtypeA__SubtypeC__SubtypeD>,
}

impl KaitaiStruct for NestedTypes3__SubtypeB {
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
        self.aCc = Box::new(NestedTypes3__SubtypeA__SubtypeCc::new(self.stream, self, _root)?);
        self.aCD = Box::new(NestedTypes3__SubtypeA__SubtypeC__SubtypeD::new(self.stream, self, _root)?);
    }
}

impl NestedTypes3__SubtypeB {
}
