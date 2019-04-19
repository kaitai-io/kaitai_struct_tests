// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct CastNested {
    pub opcodes: Vec<Box<CastNested__Opcode>>,
    pub opcodes0Str: Option<Box<CastNested__Opcode__Strval>>,
    pub opcodes0StrValue: Option<String>,
    pub opcodes1Int: Option<Box<CastNested__Opcode__Intval>>,
    pub opcodes1IntValue: Option<u8>,
}

impl KaitaiStruct for CastNested {
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
            self.opcodes.push(Box::new(CastNested__Opcode::new(self.stream, self, _root)?));
        }
    }
}

impl CastNested {
    fn opcodes0Str(&mut self) -> Box<CastNested__Opcode__Strval> {
        if let Some(x) = self.opcodes0Str {
            return x;
        }

        self.opcodes0Str = self.opcodes[0].body;
        return self.opcodes0Str;
    }
    fn opcodes0StrValue(&mut self) -> String {
        if let Some(x) = self.opcodes0StrValue {
            return x;
        }

        self.opcodes0StrValue = self.opcodes[0].body.value;
        return self.opcodes0StrValue;
    }
    fn opcodes1Int(&mut self) -> Box<CastNested__Opcode__Intval> {
        if let Some(x) = self.opcodes1Int {
            return x;
        }

        self.opcodes1Int = self.opcodes[1].body;
        return self.opcodes1Int;
    }
    fn opcodes1IntValue(&mut self) -> u8 {
        if let Some(x) = self.opcodes1IntValue {
            return x;
        }

        self.opcodes1IntValue = self.opcodes[1].body.value;
        return self.opcodes1IntValue;
    }
}
#[derive(Default)]
pub struct CastNested__Opcode {
    pub code: u8,
    pub body: Option<Box<KaitaiStruct>>,
}

impl KaitaiStruct for CastNested__Opcode {
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
                self.body = Box::new(CastNested__Opcode__Intval::new(self.stream, self, _root)?);
            },
            83 => {
                self.body = Box::new(CastNested__Opcode__Strval::new(self.stream, self, _root)?);
            },
        }
    }
}

impl CastNested__Opcode {
}
#[derive(Default)]
pub struct CastNested__Opcode__Intval {
    pub value: u8,
}

impl KaitaiStruct for CastNested__Opcode__Intval {
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

impl CastNested__Opcode__Intval {
}
#[derive(Default)]
pub struct CastNested__Opcode__Strval {
    pub value: String,
}

impl KaitaiStruct for CastNested__Opcode__Strval {
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

impl CastNested__Opcode__Strval {
}
