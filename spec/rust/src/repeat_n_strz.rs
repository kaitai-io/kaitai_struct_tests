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

pub struct RepeatNStrz {
    pub qty: u32,
    pub lines: Vec<String>*,
}

impl KaitaiStruct for RepeatNStrz {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            qty: 0,
            lines: Vec<String>*,
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
        self.lines = [];
        $n = $this->qty();
        for ($i = 0; $i < $n; $i++) {
            self.lines[] = &mut S::bytesToStr(stream->readBytesTerm(0, false, true, true), "UTF-8");
        }

        Ok(())
    }
}
