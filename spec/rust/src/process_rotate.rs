// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct ProcessRotate {
    pub buf1: Vec<u8>,
    pub buf2: Vec<u8>,
    pub key: u8,
    pub buf3: Vec<u8>,
    pub _raw_buf1: Vec<u8>,
    pub _raw_buf2: Vec<u8>,
    pub _raw_buf3: Vec<u8>,
}

impl KaitaiStruct for ProcessRotate {
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
        self._raw_buf1 = self.stream.read_bytes(5)?;
        self.buf1 = &mut S::processRotateLeft(self._raw_buf1, 3, 1);
        self._raw_buf2 = self.stream.read_bytes(5)?;
        self.buf2 = &mut S::processRotateLeft(self._raw_buf2, 8 - (3), 1);
        self.key = self.stream.read_u1()?;
        self._raw_buf3 = self.stream.read_bytes(5)?;
        self.buf3 = &mut S::processRotateLeft(self._raw_buf3, self.key, 1);
    }
}

impl ProcessRotate {
}
