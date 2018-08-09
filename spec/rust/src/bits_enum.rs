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
pub struct BitsEnum {
    pub one: Box<BitsEnum__Animal>,
    pub two: Box<BitsEnum__Animal>,
    pub three: Box<BitsEnum__Animal>,
}

impl KaitaiStruct for BitsEnum {
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
        self.one = self.stream.read_bits_int(4)?;
        self.two = self.stream.read_bits_int(8)?;
        self.three = self.stream.read_bits_int(1)?;
    }
}

impl BitsEnum {
}
enum BitsEnum__Animal {
    CAT,
    DOG,
    HORSE,
    PLATYPUS,
}
