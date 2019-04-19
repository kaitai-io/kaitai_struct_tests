// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct BitsByteAligned {
    pub one: u64,
    pub byte1: u8,
    pub two: u64,
    pub three: bool,
    pub byte2: u8,
    pub four: u64,
    pub byte3: Vec<u8>,
    pub fullByte: u64,
    pub byte4: u8,
}

impl KaitaiStruct for BitsByteAligned {
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
        self.one = self.stream.read_bits_int(6)?;
        self.stream.alignToByte();
        self.byte1 = self.stream.read_u1()?;
        self.two = self.stream.read_bits_int(3)?;
        self.three = self.stream.read_bits_int(1)? != 0;
        self.stream.alignToByte();
        self.byte2 = self.stream.read_u1()?;
        self.four = self.stream.read_bits_int(14)?;
        self.stream.alignToByte();
        self.byte3 = self.stream.read_bytes(1)?;
        self.fullByte = self.stream.read_bits_int(8)?;
        self.stream.alignToByte();
        self.byte4 = self.stream.read_u1()?;
    }
}

impl BitsByteAligned {
}
