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
pub struct IndexToParamExpr {
    pub qty: u32,
    pub sizes: Vec<u32>,
    pub blocks: Vec<Box<IndexToParamExpr__Block>>,
}

impl KaitaiStruct for IndexToParamExpr {
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
        for i in 0..self.qty {
            self.blocks.push(Box::new(IndexToParamExpr__Block::new(self.stream, self, _root)?));
        }
    }
}

impl IndexToParamExpr {
}
#[derive(Default)]
pub struct IndexToParamExpr__Block {
    pub buf: String,
}

impl KaitaiStruct for IndexToParamExpr__Block {
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

impl IndexToParamExpr__Block {
}
