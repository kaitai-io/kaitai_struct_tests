// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;
use std::vec::Vec;

#[derive(Default)]
pub struct PositionInSeq {
    pub numbers: Vec<u8>,
    pub header: Option<Box<PositionInSeq__HeaderObj>>,
}

impl KaitaiStruct for PositionInSeq {
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
        self.numbers = vec!();
        for i in 0..self.header.qty_numbers {
            self.numbers.push(self.stream.read_u1()?);
        }
    }
}

impl PositionInSeq {
    fn header(&mut self) -> Box<PositionInSeq__HeaderObj> {
        if let Some(x) = self.header {
            return x;
        }

        let _pos = self.stream.pos();
        self.stream.seek(16);
        self.header = Box::new(PositionInSeq__HeaderObj::new(self.stream, self, _root)?);
        self.stream.seek(_pos);
        return self.header;
    }
}
#[derive(Default)]
pub struct PositionInSeq__HeaderObj {
    pub qtyNumbers: u32,
}

impl KaitaiStruct for PositionInSeq__HeaderObj {
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
        self.qtyNumbers = self.stream.read_u4le()?;
    }
}

impl PositionInSeq__HeaderObj {
}
