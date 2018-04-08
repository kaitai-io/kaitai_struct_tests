// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

use std::{
    option::Option,
    boxed::Box,
    io::Result
};

use kaitai_struct::{
    KaitaiStream,
    KaitaiStruct
};

pub struct BitsSimple {
    pub testIfB1: i8,
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
}

impl KaitaiStruct for BitsSimple {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            testIfB1: 0,
            byte1: u64,
            byte2: u64,
            bitsA: bool,
            bitsB: u64,
            bitsC: u64,
            largeBits1: u64,
            spacer: u64,
            largeBits2: u64,
            normalS2: 0,
            byte8910: u64,
            byte11To14: u64,
            byte15To19: u64,
            byte20To27: u64,
        };

        s.read(stream, _parent, _root)?;

        Ok(s)
    }

    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S,
                             _parent: &Option<Box<KaitaiStruct>>,
                             _root: &Option<Box<KaitaiStruct>>)
                             -> Result<()>
        where Self: Sized {
        self.byte1 = stream->readBitsInt(8);
        self.byte2 = stream->readBitsInt(8);
        self.bitsA = stream->readBitsInt(1) != 0;
        self.bitsB = stream->readBitsInt(3);
        self.bitsC = stream->readBitsInt(4);
        self.largeBits1 = stream->readBitsInt(10);
        self.spacer = stream->readBitsInt(3);
        self.largeBits2 = stream->readBitsInt(11);
        stream->alignToByte();
        self.normalS2 = stream.read_s2be()?;
        self.byte8910 = stream->readBitsInt(24);
        self.byte11To14 = stream->readBitsInt(32);
        self.byte15To19 = stream->readBitsInt(40);
        self.byte20To27 = stream->readBitsInt(64);

        Ok(())
    }
    public function testIfB1() {
        if (self.testIfB1 !== null)
            return self.testIfB1;
        if ($this->bitsA() == false) {
            self.testIfB1 = 123;
        }
        return self.testIfB1;
    }
}
