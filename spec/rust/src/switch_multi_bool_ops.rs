// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct SwitchMultiBoolOps {
    pub opcodes: Vec<Box<SwitchMultiBoolOps__Opcode>>,
}

impl KaitaiStruct for SwitchMultiBoolOps {
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
        self.opcodes = [];
        while !self.stream.isEof() {
            self.opcodes.push(Box::new(SwitchMultiBoolOps__Opcode::new(self.stream, self, _root)?));
        }
    }
}

impl SwitchMultiBoolOps {
}
#[derive(Default)]
pub struct SwitchMultiBoolOps__Opcode {
    pub code: u8,
    pub body: u64,
}

impl KaitaiStruct for SwitchMultiBoolOps__Opcode {
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
        match if  ((self.code > 0) && (self.code <= 8) && (if self.code != 10 { true } else { false}))  { self.code } else { 0} {
            1 => {
                self.body = self.stream.read_u1()?;
            },
            2 => {
                self.body = self.stream.read_u2le()?;
            },
            4 => {
                self.body = self.stream.read_u4le()?;
            },
            8 => {
                self.body = self.stream.read_u8le()?;
            },
        }
    }
}

impl SwitchMultiBoolOps__Opcode {
}
