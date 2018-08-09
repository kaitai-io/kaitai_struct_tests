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
pub struct EnumIf {
    pub op1: Box<EnumIf__Operation>,
    pub op2: Box<EnumIf__Operation>,
    pub op3: Box<EnumIf__Operation>,
}

impl KaitaiStruct for EnumIf {
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
        self.op1 = Box::new(EnumIf__Operation::new(self.stream, self, _root)?);
        self.op2 = Box::new(EnumIf__Operation::new(self.stream, self, _root)?);
        self.op3 = Box::new(EnumIf__Operation::new(self.stream, self, _root)?);
    }
}

impl EnumIf {
}
enum EnumIf__Opcodes {
    A_STRING,
    A_TUPLE,
}
#[derive(Default)]
pub struct EnumIf__Operation {
    pub opcode: Box<EnumIf__Opcodes>,
    pub argTuple: Box<EnumIf__ArgTuple>,
    pub argStr: Box<EnumIf__ArgStr>,
}

impl KaitaiStruct for EnumIf__Operation {
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
        if self.opcode == EnumIf__Opcodes::A_TUPLE {
            self.argTuple = Box::new(EnumIf__ArgTuple::new(self.stream, self, _root)?);
        }
        if self.opcode == EnumIf__Opcodes::A_STRING {
            self.argStr = Box::new(EnumIf__ArgStr::new(self.stream, self, _root)?);
        }
    }
}

impl EnumIf__Operation {
}
#[derive(Default)]
pub struct EnumIf__ArgTuple {
    pub num1: u8,
    pub num2: u8,
}

impl KaitaiStruct for EnumIf__ArgTuple {
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

impl EnumIf__ArgTuple {
}
#[derive(Default)]
pub struct EnumIf__ArgStr {
    pub len: u8,
    pub str: String,
}

impl KaitaiStruct for EnumIf__ArgStr {
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

impl EnumIf__ArgStr {
}
