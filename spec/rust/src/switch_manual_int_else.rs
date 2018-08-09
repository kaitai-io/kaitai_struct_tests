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
pub struct SwitchManualIntElse {
    pub opcodes: Vec<Box<SwitchManualIntElse__Opcode>>,
}

impl KaitaiStruct for SwitchManualIntElse {
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
            self.opcodes.push(Box::new(SwitchManualIntElse__Opcode::new(self.stream, self, _root)?));
        }
    }
}

impl SwitchManualIntElse {
}
#[derive(Default)]
pub struct SwitchManualIntElse__Opcode {
    pub code: u8,
    pub body: Option<Box<KaitaiStruct>>,
}

impl KaitaiStruct for SwitchManualIntElse__Opcode {
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
        match self.code {
            73 => {
                self.body = Box::new(SwitchManualIntElse__Opcode__Intval::new(self.stream, self, _root)?);
            },
            83 => {
                self.body = Box::new(SwitchManualIntElse__Opcode__Strval::new(self.stream, self, _root)?);
            },
            _ => {
                self.body = Box::new(SwitchManualIntElse__Opcode__Noneval::new(self.stream, self, _root)?);
            }
        }
    }
}

impl SwitchManualIntElse__Opcode {
}
#[derive(Default)]
pub struct SwitchManualIntElse__Opcode__Intval {
    pub value: u8,
}

impl KaitaiStruct for SwitchManualIntElse__Opcode__Intval {
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
        self.value = self.stream.read_u1()?;
    }
}

impl SwitchManualIntElse__Opcode__Intval {
}
#[derive(Default)]
pub struct SwitchManualIntElse__Opcode__Strval {
    pub value: String,
}

impl KaitaiStruct for SwitchManualIntElse__Opcode__Strval {
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
        self.value = String::from_utf8_lossy(self.stream.read_bytes_term(0, false, true, true)?);
    }
}

impl SwitchManualIntElse__Opcode__Strval {
}
#[derive(Default)]
pub struct SwitchManualIntElse__Opcode__Noneval {
    pub filler: u32,
}

impl KaitaiStruct for SwitchManualIntElse__Opcode__Noneval {
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
        self.filler = self.stream.read_u4le()?;
    }
}

impl SwitchManualIntElse__Opcode__Noneval {
}
