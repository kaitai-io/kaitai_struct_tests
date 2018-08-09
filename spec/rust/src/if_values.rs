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
pub struct IfValues {
    pub codes: Vec<Box<IfValues__Code>>,
}

impl KaitaiStruct for IfValues {
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
        self.codes = vec!();
        for i in 0..3 {
            self.codes.push(Box::new(IfValues__Code::new(self.stream, self, _root)?));
        }
    }
}

impl IfValues {
}
#[derive(Default)]
pub struct IfValues__Code {
    pub opcode: u8,
    pub halfOpcode: Option<i32>,
}

impl KaitaiStruct for IfValues__Code {
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
    }
}

impl IfValues__Code {
    fn halfOpcode(&mut self) -> i32 {
        if let Some(x) = self.halfOpcode {
            return x;
        }

        if self.opcode % 2 == 0 {
            self.halfOpcode = self.opcode / 2;
        }
        return self.halfOpcode;
    }
}
