// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct StrEncodingsDefault {
    pub lenOf1: u16,
    pub str1: String,
    pub rest: Box<StrEncodingsDefault__Subtype>,
}

impl KaitaiStruct for StrEncodingsDefault {
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
        self.str1 = panic!("Unimplemented encoding for bytesToStr: {}", "UTF-8");
        self.rest = Box::new(StrEncodingsDefault__Subtype::new(self.stream, self, _root)?);
    }
}

impl StrEncodingsDefault {
}
#[derive(Default)]
pub struct StrEncodingsDefault__Subtype {
    pub lenOf2: u16,
    pub str2: String,
    pub lenOf3: u16,
    pub str3: String,
    pub lenOf4: u16,
    pub str4: String,
}

impl KaitaiStruct for StrEncodingsDefault__Subtype {
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
        self.lenOf2 = self.stream.read_u2le()?;
        self.str2 = panic!("Unimplemented encoding for bytesToStr: {}", "UTF-8");
        self.lenOf3 = self.stream.read_u2le()?;
        self.str3 = panic!("Unimplemented encoding for bytesToStr: {}", "SJIS");
        self.lenOf4 = self.stream.read_u2le()?;
        self.str4 = panic!("Unimplemented encoding for bytesToStr: {}", "CP437");
    }
}

impl StrEncodingsDefault__Subtype {
}
