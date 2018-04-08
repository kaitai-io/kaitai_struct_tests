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

pub struct TermBytes {
    pub s1: String,
    pub s2: String,
    pub s3: String,
}

impl KaitaiStruct for TermBytes {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            s1: String,
            s2: String,
            s3: String,
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
        self.s1 = stream->readBytesTerm(124, false, true, true);
        self.s2 = stream->readBytesTerm(124, false, false, true);
        self.s3 = stream->readBytesTerm(64, true, true, true);

        Ok(())
    }
}
