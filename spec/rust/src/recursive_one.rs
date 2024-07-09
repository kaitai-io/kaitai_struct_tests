// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct RecursiveOne {
    pub one: u8,
    pub next: Option<Box<KaitaiStruct>>,
}

impl KaitaiStruct for RecursiveOne {
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
        self.one = self.stream.read_u1()?;
        match (self.one & 3) {
            0 => {
                self.next = Box::new(RecursiveOne::new(self.stream, self, _root)?);
            },
            1 => {
                self.next = Box::new(RecursiveOne::new(self.stream, self, _root)?);
            },
            2 => {
                self.next = Box::new(RecursiveOne::new(self.stream, self, _root)?);
            },
            3 => {
                self.next = Box::new(RecursiveOne__Fini::new(self.stream, self, _root)?);
            },
        }
    }
}

impl RecursiveOne {
}
#[derive(Default)]
pub struct RecursiveOne__Fini {
    pub finisher: u16,
}

impl KaitaiStruct for RecursiveOne__Fini {
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
        self.finisher = self.stream.read_u2le()?;
    }
}

impl RecursiveOne__Fini {
}
