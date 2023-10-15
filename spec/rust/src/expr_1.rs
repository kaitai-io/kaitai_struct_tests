// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct Expr1 {
    pub lenOf1: u16,
    pub str1: String,
    pub lenOf1Mod: Option<i32>,
    pub str1Len: Option<i32>,
}

impl KaitaiStruct for Expr1 {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s: Self = Default::default();

        s.stream = stream;
        s.read(stream, _parent, _root)?;

        Ok(s)
    }


    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S,
                             _parent: &Option<Box<KaitaiStruct>>,
                             _root: &Option<Box<KaitaiStruct>>)
                             -> Result<()>
        where Self: Sized {
        self.lenOf1 = self.stream.read_u2le()?;
        self.str1 = String::from_utf8_lossy(self.stream.read_bytes(self.len_of_1_mod)?);
    }
}

impl Expr1 {
    fn lenOf1Mod(&mut self) -> i32 {
        if let Some(x) = self.lenOf1Mod {
            return x;
        }

        self.lenOf1Mod = (self.len_of_1 - 2);
        return self.lenOf1Mod;
    }
    fn str1Len(&mut self) -> i32 {
        if let Some(x) = self.str1Len {
            return x;
        }

        self.str1Len = self.str1.len();
        return self.str1Len;
    }
}
