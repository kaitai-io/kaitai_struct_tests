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

pub struct ProcessXor4Const {
    pub key: String,
    pub buf: String,
    pub _raw_buf: String,
}

impl KaitaiStruct for ProcessXor4Const {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            key: String,
            buf: String,
            _raw_buf: String,
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
        self.key = stream->readBytes(4);
        self._raw_buf = stream->readBytesFull();
        self.buf = &mut S::processXorMany(self._raw_buf, "\xEC\xBB\xA3\x14");

        Ok(())
    }
}
