// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct TermStrz {
    pub s1: String,
    pub s2: String,
    pub s3: String,
}

impl KaitaiStruct for TermStrz {
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
        self.s1 = panic!("Unimplemented encoding for bytesToStr: {}", "UTF-8");
        self.s2 = panic!("Unimplemented encoding for bytesToStr: {}", "UTF-8");
        self.s3 = panic!("Unimplemented encoding for bytesToStr: {}", "UTF-8");
    }
}

impl TermStrz {
}
