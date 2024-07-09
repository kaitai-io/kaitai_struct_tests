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
pub struct IndexSizes {
    pub qty: u32,
    pub sizes: Vec<u32>,
    pub bufs: Vec<String>,
}

impl KaitaiStruct for IndexSizes {
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
        self.bufs = vec!();
        for i in 0..self.qty {
            self.bufs.push(String::from_utf8_lossy(self.stream.read_bytes(self.sizes[i])?));
        }
    }
}

impl IndexSizes {
}
