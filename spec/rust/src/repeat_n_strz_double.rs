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

pub struct RepeatNStrzDouble {
    pub qty: u32,
    pub lines1: Vec<String>*,
    pub lines2: Vec<String>*,
}

impl KaitaiStruct for RepeatNStrzDouble {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            qty: 0,
            lines1: Vec<String>*,
            lines2: Vec<String>*,
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
        self.qty = stream.read_u4le()?;
        self.lines1 = [];
        $n = intval($this->qty() / 2);
        for ($i = 0; $i < $n; $i++) {
            self.lines1[] = &mut S::bytesToStr(stream->readBytesTerm(0, false, true, true), "UTF-8");
        }
        self.lines2 = [];
        $n = intval($this->qty() / 2);
        for ($i = 0; $i < $n; $i++) {
            self.lines2[] = &mut S::bytesToStr(stream->readBytesTerm(0, false, true, true), "UTF-8");
        }

        Ok(())
    }
}
