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

pub struct StrEncodings {
    pub lenOf1: u16,
    pub str1: String,
    pub lenOf2: u16,
    pub str2: String,
    pub lenOf3: u16,
    pub str3: String,
    pub lenOf4: u16,
    pub str4: String,
}

impl KaitaiStruct for StrEncodings {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            lenOf1: 0,
            str1: String,
            lenOf2: 0,
            str2: String,
            lenOf3: 0,
            str3: String,
            lenOf4: 0,
            str4: String,
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
        self.lenOf1 = stream.read_u2le()?;
        self.str1 = &mut S::bytesToStr(stream->readBytes($this->lenOf1()), "ASCII");
        self.lenOf2 = stream.read_u2le()?;
        self.str2 = &mut S::bytesToStr(stream->readBytes($this->lenOf2()), "UTF-8");
        self.lenOf3 = stream.read_u2le()?;
        self.str3 = &mut S::bytesToStr(stream->readBytes($this->lenOf3()), "SJIS");
        self.lenOf4 = stream.read_u2le()?;
        self.str4 = &mut S::bytesToStr(stream->readBytes($this->lenOf4()), "CP437");

        Ok(())
    }
}
