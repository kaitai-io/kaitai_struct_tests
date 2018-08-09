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
pub struct ExprIoEof {
    pub substream1: Box<ExprIoEof__OneOrTwo>,
    pub substream2: Box<ExprIoEof__OneOrTwo>,
    pub _raw_substream1: Vec<u8>,
    pub _raw_substream2: Vec<u8>,
}

impl KaitaiStruct for ExprIoEof {
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
        self._raw_substream1 = self.stream.read_bytes(4)?;
        let mut io = Cursor::new(self._raw_substream1);
        self.substream1 = Box::new(ExprIoEof__OneOrTwo::new(self.stream, self, _root)?);
        self._raw_substream2 = self.stream.read_bytes(8)?;
        let mut io = Cursor::new(self._raw_substream2);
        self.substream2 = Box::new(ExprIoEof__OneOrTwo::new(self.stream, self, _root)?);
    }
}

impl ExprIoEof {
}
#[derive(Default)]
pub struct ExprIoEof__OneOrTwo {
    pub one: u32,
    pub two: u32,
    pub reflectEof: Option<bool>,
}

impl KaitaiStruct for ExprIoEof__OneOrTwo {
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
        self.one = self.stream.read_u4le()?;
        if !(self._io.is_eof) {
            self.two = self.stream.read_u4le()?;
        }
    }
}

impl ExprIoEof__OneOrTwo {
    fn reflectEof(&mut self) -> bool {
        if let Some(x) = self.reflectEof {
            return x;
        }

        self.reflectEof = self._io.is_eof;
        return self.reflectEof;
    }
}
