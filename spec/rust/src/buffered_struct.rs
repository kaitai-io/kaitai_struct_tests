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
pub struct BufferedStruct {
    pub len1: u32,
    pub block1: Box<BufferedStruct__Block>,
    pub len2: u32,
    pub block2: Box<BufferedStruct__Block>,
    pub finisher: u32,
    pub _raw_block1: Vec<u8>,
    pub _raw_block2: Vec<u8>,
}

impl KaitaiStruct for BufferedStruct {
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
        self.len1 = self.stream.read_u4le()?;
        self._raw_block1 = self.stream.read_bytes(self.len1)?;
        let mut io = Cursor::new(self._raw_block1);
        self.block1 = Box::new(BufferedStruct__Block::new(self.stream, self, _root)?);
        self.len2 = self.stream.read_u4le()?;
        self._raw_block2 = self.stream.read_bytes(self.len2)?;
        let mut io = Cursor::new(self._raw_block2);
        self.block2 = Box::new(BufferedStruct__Block::new(self.stream, self, _root)?);
        self.finisher = self.stream.read_u4le()?;
    }
}

impl BufferedStruct {
}
#[derive(Default)]
pub struct BufferedStruct__Block {
    pub number1: u32,
    pub number2: u32,
}

impl KaitaiStruct for BufferedStruct__Block {
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
        self.number1 = self.stream.read_u4le()?;
        self.number2 = self.stream.read_u4le()?;
    }
}

impl BufferedStruct__Block {
}
