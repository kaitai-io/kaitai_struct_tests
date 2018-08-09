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
pub struct IndexToParamUntil {
    pub qty: u32,
    pub sizes: Vec<u32>,
    pub blocks: Vec<Box<IndexToParamUntil__Block>>,
}

impl KaitaiStruct for IndexToParamUntil {
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
        self.qty = self.stream.read_u4le()?;
        self.sizes = vec!();
        for i in 0..self.qty {
            self.sizes.push(self.stream.read_u4le()?);
        }
        self.blocks = vec!();
        while {
            let tmpa = Box::new(IndexToParamUntil__Block::new(self.stream, self, _root)?);
            self.blocks.append(Box::new(IndexToParamUntil__Block::new(self.stream, self, _root)?));
            !(self._io.is_eof)
        } { }
    }
}

impl IndexToParamUntil {
}
#[derive(Default)]
pub struct IndexToParamUntil__Block {
    pub buf: String,
}

impl KaitaiStruct for IndexToParamUntil__Block {
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
        self.buf = String::from_utf8_lossy(self.stream.read_bytes(self._root.sizes[self.idx])?);
    }
}

impl IndexToParamUntil__Block {
}
