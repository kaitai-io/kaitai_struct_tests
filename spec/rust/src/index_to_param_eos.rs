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
pub struct IndexToParamEos {
    pub qty: u32,
    pub sizes: Vec<u32>,
    pub blocks: Vec<Box<IndexToParamEos__Block>>,
}

impl KaitaiStruct for IndexToParamEos {
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
        self.blocks = [];
        while !self.stream.isEof() {
            self.blocks.push(Box::new(IndexToParamEos__Block::new(self.stream, self, _root)?));
        }
    }
}

impl IndexToParamEos {
}
#[derive(Default)]
pub struct IndexToParamEos__Block {
    pub buf: String,
}

impl KaitaiStruct for IndexToParamEos__Block {
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

impl IndexToParamEos__Block {
}
