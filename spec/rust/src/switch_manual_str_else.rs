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
pub struct SwitchManualStrElse {
    pub opcodes: Vec<Box<SwitchManualStrElse__Opcode>>,
}

impl KaitaiStruct for SwitchManualStrElse {
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
            self.opcodes.push(Box::new(SwitchManualStrElse__Opcode::new(self.stream, self, _root)?));
        }
    }
}

impl SwitchManualStrElse {
}
#[derive(Default)]
pub struct SwitchManualStrElse__Opcode {
    pub code: String,
    pub body: Option<Box<KaitaiStruct>>,
}

impl KaitaiStruct for SwitchManualStrElse__Opcode {
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
        self.code = String::from_utf8_lossy(self.stream.read_bytes(1)?);
        match self.code {
            "I" => {
                self.body = Box::new(SwitchManualStrElse__Opcode__Intval::new(self.stream, self, _root)?);
            },
            "S" => {
                self.body = Box::new(SwitchManualStrElse__Opcode__Strval::new(self.stream, self, _root)?);
            },
            _ => {
                self.body = Box::new(SwitchManualStrElse__Opcode__Noneval::new(self.stream, self, _root)?);
            }
        }
    }
}

impl SwitchManualStrElse__Opcode {
}
#[derive(Default)]
pub struct SwitchManualStrElse__Opcode__Intval {
    pub value: u8,
}

impl KaitaiStruct for SwitchManualStrElse__Opcode__Intval {
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

impl SwitchManualStrElse__Opcode__Intval {
}
#[derive(Default)]
pub struct SwitchManualStrElse__Opcode__Strval {
    pub value: String,
}

impl KaitaiStruct for SwitchManualStrElse__Opcode__Strval {
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

impl SwitchManualStrElse__Opcode__Strval {
}
#[derive(Default)]
pub struct SwitchManualStrElse__Opcode__Noneval {
    pub filler: u32,
}

impl KaitaiStruct for SwitchManualStrElse__Opcode__Noneval {
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

impl SwitchManualStrElse__Opcode__Noneval {
}
