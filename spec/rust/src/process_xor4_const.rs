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
pub struct ProcessXor4Const {
    pub key: Vec<u8>,
    pub buf: Vec<u8>,
    pub _raw_buf: Vec<u8>,
}

impl KaitaiStruct for ProcessXor4Const {
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
        self.key = self.stream.read_bytes(4)?;
        self._raw_buf = self.stream.read_bytes_full()?;
        self.buf = &mut S::processXorMany(self._raw_buf, vec!([0xec, 0xbb, 0xa3, 0x14]));
    }
}

impl ProcessXor4Const {
}
