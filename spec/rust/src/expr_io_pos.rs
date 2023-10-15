// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;
use std::io::Cursor;

#[derive(Default)]
pub struct ExprIoPos {
    pub substream1: Box<ExprIoPos__AllPlusNumber>,
    pub substream2: Box<ExprIoPos__AllPlusNumber>,
    pub _raw_substream1: Vec<u8>,
    pub _raw_substream2: Vec<u8>,
}

impl KaitaiStruct for ExprIoPos {
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
        self._raw_substream1 = self.stream.read_bytes(16)?;
        let mut io = Cursor::new(self._raw_substream1);
        self.substream1 = Box::new(ExprIoPos__AllPlusNumber::new(self.stream, self, _root)?);
        self._raw_substream2 = self.stream.read_bytes(14)?;
        let mut io = Cursor::new(self._raw_substream2);
        self.substream2 = Box::new(ExprIoPos__AllPlusNumber::new(self.stream, self, _root)?);
    }
}

impl ExprIoPos {
}
#[derive(Default)]
pub struct ExprIoPos__AllPlusNumber {
    pub myStr: String,
    pub body: Vec<u8>,
    pub number: u16,
}

impl KaitaiStruct for ExprIoPos__AllPlusNumber {
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
        self.myStr = panic!("Unimplemented encoding for bytesToStr: {}", "UTF-8");
        self.body = self.stream.read_bytes(((self._io.size - self._io.pos) - 2))?;
        self.number = self.stream.read_u2le()?;
    }
}

impl ExprIoPos__AllPlusNumber {
}
