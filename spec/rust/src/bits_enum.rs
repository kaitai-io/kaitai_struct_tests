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

pub struct BitsEnum {
pub struct Animal {
    pub one: ,
    pub two: ,
    pub three: ,
}

impl KaitaiStruct for BitsEnum {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            one: ,
            two: ,
            three: ,
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
        self.one = stream->readBitsInt(4);
        self.two = stream->readBitsInt(8);
        self.three = stream->readBitsInt(1);

        Ok(())
    }
}
const CAT = 0;
const DOG = 1;
const HORSE = 4;
const PLATYPUS = 5;
}
