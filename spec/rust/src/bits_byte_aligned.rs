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

pub struct BitsByteAligned {
    pub one: u64,
    pub byte1: u8,
    pub two: u64,
    pub three: bool,
    pub byte2: u8,
    pub four: u64,
    pub byte3: String,
    pub fullByte: u64,
    pub byte4: u8,
}

impl KaitaiStruct for BitsByteAligned {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            one: u64,
            byte1: 0,
            two: u64,
            three: bool,
            byte2: 0,
            four: u64,
            byte3: String,
            fullByte: u64,
            byte4: 0,
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
        self.one = stream->readBitsInt(6);
        stream->alignToByte();
        self.byte1 = stream.read_u1()?;
        self.two = stream->readBitsInt(3);
        self.three = stream->readBitsInt(1) != 0;
        stream->alignToByte();
        self.byte2 = stream.read_u1()?;
        self.four = stream->readBitsInt(14);
        stream->alignToByte();
        self.byte3 = stream->readBytes(1);
        self.fullByte = stream->readBitsInt(8);
        stream->alignToByte();
        self.byte4 = stream.read_u1()?;

        Ok(())
    }
}
