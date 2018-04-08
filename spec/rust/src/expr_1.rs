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

pub struct Expr1 {
    pub lenOf1Mod: i32,
    pub str1Len: i32,
    pub lenOf1: u16,
    pub str1: String,
}

impl KaitaiStruct for Expr1 {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            lenOf1Mod: i32,
            str1Len: i32,
            lenOf1: 0,
            str1: String,
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
        self.str1 = &mut S::bytesToStr(stream->readBytes($this->lenOf1Mod()), "ASCII");

        Ok(())
    }
    public function lenOf1Mod() {
        if (self.lenOf1Mod !== null)
            return self.lenOf1Mod;
        self.lenOf1Mod = ($this->lenOf1() - 2);
        return self.lenOf1Mod;
    }
    public function str1Len() {
        if (self.str1Len !== null)
            return self.str1Len;
        self.str1Len = strlen($this->str1());
        return self.str1Len;
    }
}
