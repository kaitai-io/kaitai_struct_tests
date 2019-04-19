// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;
use std::vec::Vec;

#[derive(Default)]
pub struct RepeatUntilS4 {
    pub entries: Vec<i32>,
    pub afterall: String,
}

impl KaitaiStruct for RepeatUntilS4 {
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
        self.entries = vec!();
        while {
            let tmpa = self.stream.read_s4le()?;
            self.entries.append(self.stream.read_s4le()?);
            !(tmpa == -1)
        } { }
        self.afterall = String::from_utf8_lossy(self.stream.read_bytes_term(0, false, true, true)?);
    }
}

impl RepeatUntilS4 {
}
