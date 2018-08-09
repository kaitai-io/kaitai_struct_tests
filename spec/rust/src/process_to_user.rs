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
pub struct ProcessToUser {
    pub buf1: Box<ProcessToUser__JustStr>,
    pub _raw_buf1: Vec<u8>,
    pub _raw__raw_buf1: Vec<u8>,
}

impl KaitaiStruct for ProcessToUser {
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
        self._raw__raw_buf1 = self.stream.read_bytes(5)?;
        self._raw_buf1 = &mut S::processRotateLeft(self._raw__raw_buf1, 3, 1);
        let mut io = Cursor::new(self._raw_buf1);
        self.buf1 = Box::new(ProcessToUser__JustStr::new(self.stream, self, _root)?);
    }
}

impl ProcessToUser {
}
#[derive(Default)]
pub struct ProcessToUser__JustStr {
    pub str: String,
}

impl KaitaiStruct for ProcessToUser__JustStr {
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
        self.str = panic!("Unimplemented encoding for bytesToStr: {}", "UTF-8");
    }
}

impl ProcessToUser__JustStr {
}
