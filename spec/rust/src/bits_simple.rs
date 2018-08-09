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
pub struct BitsSimple {
    pub byte1: u64,
    pub byte2: u64,
    pub bitsA: bool,
    pub bitsB: u64,
    pub bitsC: u64,
    pub largeBits1: u64,
    pub spacer: u64,
    pub largeBits2: u64,
    pub normalS2: i16,
    pub byte8910: u64,
    pub byte11To14: u64,
    pub byte15To19: u64,
    pub byte20To27: u64,
    pub testIfB1: Option<i8>,
}

impl KaitaiStruct for BitsSimple {
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
        self.byte1 = self.stream.read_bits_int(8)?;
        self.byte2 = self.stream.read_bits_int(8)?;
        self.bitsA = self.stream.read_bits_int(1)? != 0;
        self.bitsB = self.stream.read_bits_int(3)?;
        self.bitsC = self.stream.read_bits_int(4)?;
        self.largeBits1 = self.stream.read_bits_int(10)?;
        self.spacer = self.stream.read_bits_int(3)?;
        self.largeBits2 = self.stream.read_bits_int(11)?;
        self.stream.alignToByte();
        self.normalS2 = self.stream.read_s2be()?;
        self.byte8910 = self.stream.read_bits_int(24)?;
        self.byte11To14 = self.stream.read_bits_int(32)?;
        self.byte15To19 = self.stream.read_bits_int(40)?;
        self.byte20To27 = self.stream.read_bits_int(64)?;
    }
}

impl BitsSimple {
    fn testIfB1(&mut self) -> i8 {
        if let Some(x) = self.testIfB1 {
            return x;
        }

        if self.bits_a == false {
            self.testIfB1 = 123;
        }
        return self.testIfB1;
    }
}
