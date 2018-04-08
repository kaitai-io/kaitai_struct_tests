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

pub struct StrPadTerm {
    pub strPad: String,
    pub strTerm: String,
    pub strTermAndPad: String,
    pub strTermInclude: String,
}

impl KaitaiStruct for StrPadTerm {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            strPad: String,
            strTerm: String,
            strTermAndPad: String,
            strTermInclude: String,
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
        self.strPad = &mut S::bytesToStr(&mut S::bytesStripRight(stream->readBytes(20), 64), "UTF-8");
        self.strTerm = &mut S::bytesToStr(&mut S::bytesTerminate(stream->readBytes(20), 64, false), "UTF-8");
        self.strTermAndPad = &mut S::bytesToStr(&mut S::bytesTerminate(&mut S::bytesStripRight(stream->readBytes(20), 43), 64, false), "UTF-8");
        self.strTermInclude = &mut S::bytesToStr(&mut S::bytesTerminate(stream->readBytes(20), 64, true), "UTF-8");

        Ok(())
    }
}
