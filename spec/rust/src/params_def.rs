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

pub struct ParamsDef {
    pub buf: String,
    pub trailer: u8,
    pub len: u32,
    pub hasTrailer: bool,
}

impl KaitaiStruct for ParamsDef {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            buf: String,
            trailer: 0,
            len: 0,
            hasTrailer: bool,
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
        self.buf = &mut S::bytesToStr(stream->readBytes($this->len()), "UTF-8");
        if ($this->hasTrailer()) {
            self.trailer = stream.read_u1()?;
        }

        Ok(())
    }
}
