// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct SwitchCast {
    pub opcodes: Vec<Box<SwitchCast__Opcode>>,
    pub firstObj: Option<Box<SwitchCast__Strval>>,
    pub secondVal: Option<u8>,
    pub errCast: Option<Box<SwitchCast__Strval>>,
}

impl KaitaiStruct for SwitchCast {
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
            self.opcodes.push(Box::new(SwitchCast__Opcode::new(self.stream, self, _root)?));
        }
    }
}

impl SwitchCast {
    fn firstObj(&mut self) -> Box<SwitchCast__Strval> {
        if let Some(x) = self.firstObj {
            return x;
        }

        self.firstObj = self.opcodes[0].body;
        return self.firstObj;
    }
    fn secondVal(&mut self) -> u8 {
        if let Some(x) = self.secondVal {
            return x;
        }

        self.secondVal = self.opcodes[1].body.value;
        return self.secondVal;
    }
    fn errCast(&mut self) -> Box<SwitchCast__Strval> {
        if let Some(x) = self.errCast {
            return x;
        }

        self.errCast = self.opcodes[2].body;
        return self.errCast;
    }
}
#[derive(Default)]
pub struct SwitchCast__Opcode {
    pub code: u8,
    pub body: Option<Box<KaitaiStruct>>,
}

impl KaitaiStruct for SwitchCast__Opcode {
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
                self.body = Box::new(SwitchCast__Intval::new(self.stream, self, _root)?);
            },
            83 => {
                self.body = Box::new(SwitchCast__Strval::new(self.stream, self, _root)?);
            },
        }
    }
}

impl SwitchCast__Opcode {
}
#[derive(Default)]
pub struct SwitchCast__Intval {
    pub value: u8,
}

impl KaitaiStruct for SwitchCast__Intval {
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

impl SwitchCast__Intval {
}
#[derive(Default)]
pub struct SwitchCast__Strval {
    pub value: String,
}

impl KaitaiStruct for SwitchCast__Strval {
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

impl SwitchCast__Strval {
}
