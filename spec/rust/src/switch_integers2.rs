// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct SwitchIntegers2 {
    pub code: u8,
    pub len: u64,
    pub ham: Vec<u8>,
    pub padding: u8,
    pub lenModStr: Option<String>,
}

impl KaitaiStruct for SwitchIntegers2 {
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
        self.code = self.stream.read_u1()?;
        match self.code {
            1 => {
                self.len = self.stream.read_u1()?;
            },
            2 => {
                self.len = self.stream.read_u2le()?;
            },
            4 => {
                self.len = self.stream.read_u4le()?;
            },
            8 => {
                self.len = self.stream.read_u8le()?;
            },
        }
        self.ham = self.stream.read_bytes(self.len)?;
        if self.len > 3 {
            self.padding = self.stream.read_u1()?;
        }
    }
}

impl SwitchIntegers2 {
    fn lenModStr(&mut self) -> String {
        if let Some(x) = self.lenModStr {
            return x;
        }

        self.lenModStr = ((self.len * 2) - 1).to_string();
        return self.lenModStr;
    }
}
