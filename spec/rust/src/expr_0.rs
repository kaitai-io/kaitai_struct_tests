// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct Expr0 {
    pub lenOf1: u16,
    pub mustBeF7: Option<i32>,
    pub mustBeAbc123: Option<String>,
}

impl KaitaiStruct for Expr0 {
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
    }
}

impl Expr0 {
    fn mustBeF7(&mut self) -> i32 {
        if let Some(x) = self.mustBeF7 {
            return x;
        }

        self.mustBeF7 = (7 + 240);
        return self.mustBeF7;
    }
    fn mustBeAbc123(&mut self) -> String {
        if let Some(x) = self.mustBeAbc123 {
            return x;
        }

        self.mustBeAbc123 = format!("{}{}", "abc", "123");
        return self.mustBeAbc123;
    }
}
