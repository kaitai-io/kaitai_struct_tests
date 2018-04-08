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

pub struct RepeatUntilS4 {
    pub entries: Vec<i32>*,
    pub afterall: String,
}

impl KaitaiStruct for RepeatUntilS4 {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            entries: Vec<i32>*,
            afterall: String,
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
        self.entries = [];
        $i = 0;
        do {
            $_ = stream.read_s4le()?;
            self.entries[] = $_;
            $i++;
        } while (!($_ == -1));
        self.afterall = &mut S::bytesToStr(stream->readBytesTerm(0, false, true, true), "ASCII");

        Ok(())
    }
}
