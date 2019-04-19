// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct IfStruct {
    pub op1: Box<IfStruct__Operation>,
    pub op2: Box<IfStruct__Operation>,
    pub op3: Box<IfStruct__Operation>,
}

impl KaitaiStruct for IfStruct {
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
        self.op1 = Box::new(IfStruct__Operation::new(self.stream, self, _root)?);
        self.op2 = Box::new(IfStruct__Operation::new(self.stream, self, _root)?);
        self.op3 = Box::new(IfStruct__Operation::new(self.stream, self, _root)?);
    }
}

impl IfStruct {
}
#[derive(Default)]
pub struct IfStruct__Operation {
    pub opcode: u8,
    pub argTuple: Box<IfStruct__ArgTuple>,
    pub argStr: Box<IfStruct__ArgStr>,
}

impl KaitaiStruct for IfStruct__Operation {
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
        self.opcode = self.stream.read_u1()?;
        if self.opcode == 84 {
            self.argTuple = Box::new(IfStruct__ArgTuple::new(self.stream, self, _root)?);
        }
        if self.opcode == 83 {
            self.argStr = Box::new(IfStruct__ArgStr::new(self.stream, self, _root)?);
        }
    }
}

impl IfStruct__Operation {
}
#[derive(Default)]
pub struct IfStruct__ArgTuple {
    pub num1: u8,
    pub num2: u8,
}

impl KaitaiStruct for IfStruct__ArgTuple {
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
        self.num1 = self.stream.read_u1()?;
        self.num2 = self.stream.read_u1()?;
    }
}

impl IfStruct__ArgTuple {
}
#[derive(Default)]
pub struct IfStruct__ArgStr {
    pub len: u8,
    pub str: String,
}

impl KaitaiStruct for IfStruct__ArgStr {
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
        self.len = self.stream.read_u1()?;
        self.str = panic!("Unimplemented encoding for bytesToStr: {}", "UTF-8");
    }
}

impl IfStruct__ArgStr {
}
