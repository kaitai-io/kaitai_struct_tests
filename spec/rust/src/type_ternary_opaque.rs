// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;
use term_strz::TermStrz;
use std::io::Cursor;

#[derive(Default)]
pub struct TypeTernaryOpaque {
    pub difWoHack: Box<TermStrz>,
    pub difWithHack: Box<TermStrz>,
    pub _raw_difWoHack: Vec<u8>,
    pub _raw_difWithHack: Vec<u8>,
    pub _raw__raw_difWithHack: Vec<u8>,
    pub isHack: Option<bool>,
    pub dif: Option<Box<TermStrz>>,
}

impl KaitaiStruct for TypeTernaryOpaque {
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
        if !(self.is_hack) {
            self._raw_difWoHack = self.stream.read_bytes(12)?;
            let mut io = Cursor::new(self._raw_difWoHack);
            self.difWoHack = Box::new(TermStrz::new(self.stream, self, _root)?);
        }
        if self.is_hack {
            self._raw__raw_difWithHack = self.stream.read_bytes(12)?;
            self._raw_difWithHack = &mut S::processXorOne(self._raw__raw_difWithHack, 3);
            let mut io = Cursor::new(self._raw_difWithHack);
            self.difWithHack = Box::new(TermStrz::new(self.stream, self, _root)?);
        }
    }
}

impl TypeTernaryOpaque {
    fn isHack(&mut self) -> bool {
        if let Some(x) = self.isHack {
            return x;
        }

        self.isHack = false;
        return self.isHack;
    }
    fn dif(&mut self) -> Box<TermStrz> {
        if let Some(x) = self.dif {
            return x;
        }

        self.dif = if !(self.is_hack) { self.dif_wo_hack } else { self.dif_with_hack};
        return self.dif;
    }
}
