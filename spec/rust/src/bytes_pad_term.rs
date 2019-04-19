// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct BytesPadTerm {
    pub strPad: Vec<u8>,
    pub strTerm: Vec<u8>,
    pub strTermAndPad: Vec<u8>,
    pub strTermInclude: Vec<u8>,
}

impl KaitaiStruct for BytesPadTerm {
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
        self.strPad = &mut S::bytesStripRight(self.stream.read_bytes(20)?, 64);
        self.strTerm = &mut S::bytesTerminate(self.stream.read_bytes(20)?, 64, false);
        self.strTermAndPad = &mut S::bytesTerminate(&mut S::bytesStripRight(self.stream.read_bytes(20)?, 43), 64, false);
        self.strTermInclude = &mut S::bytesTerminate(self.stream.read_bytes(20)?, 64, true);
    }
}

impl BytesPadTerm {
}
