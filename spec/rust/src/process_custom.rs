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

pub struct ProcessCustom {
    pub buf1: String,
    pub buf2: String,
    pub key: u8,
    pub buf3: String,
    pub _raw_buf1: String,
    pub _raw_buf2: String,
    pub _raw_buf3: String,
}

impl KaitaiStruct for ProcessCustom {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            buf1: String,
            buf2: String,
            key: 0,
            buf3: String,
            _raw_buf1: String,
            _raw_buf2: String,
            _raw_buf3: String,
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
        self._raw_buf1 = stream->readBytes(5);
        $_process = new MyCustomFx(7, true, "\x20\x30\x40");
        self.buf1 = $_process->decode(self._raw_buf1);
        self._raw_buf2 = stream->readBytes(5);
        $_process = new \Nested\Deeply\CustomFx(7);
        self.buf2 = $_process->decode(self._raw_buf2);
        self.key = stream.read_u1()?;
        self._raw_buf3 = stream->readBytes(5);
        $_process = new MyCustomFx($this->key(), false, "\x00");
        self.buf3 = $_process->decode(self._raw_buf3);

        Ok(())
    }
}
