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

pub struct ProcessXorConst {
    pub key: u8,
    pub buf: String,
    pub _raw_buf: String,
}

impl KaitaiStruct for ProcessXorConst {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            key: 0,
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
        self.key = stream.read_u1()?;
        self._raw_buf = stream->readBytesFull();
        self.buf = &mut S::processXorOne(self._raw_buf, 255);

        Ok(())
    }
}
