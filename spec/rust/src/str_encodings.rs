// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::io::Cursor;
use std::vec::Vec;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
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
        self.str1 = String::from_utf8_lossy(self.stream.read_bytes(self.len_of_1)?);
        self.lenOf2 = self.stream.read_u2le()?;
        self.str2 = panic!("Unimplemented encoding for bytesToStr: {}", "UTF-8");
        self.lenOf3 = self.stream.read_u2le()?;
        self.str3 = panic!("Unimplemented encoding for bytesToStr: {}", "SJIS");
        self.lenOf4 = self.stream.read_u2le()?;
        self.str4 = panic!("Unimplemented encoding for bytesToStr: {}", "CP437");
    }
}

impl StrEncodings {
}
