// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

use std::{
    option::Option,
    boxed::Box,
    io::Result
};

use kaitai_struct::{
    KaitaiStream,
    KaitaiStruct
};

pub struct IndexSizes {
    pub qty: u32,
    pub sizes: Vec<u32>*,
    pub bufs: Vec<String>*,
}

impl KaitaiStruct for IndexSizes {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            qty: 0,
            sizes: Vec<u32>*,
            bufs: Vec<String>*,
        };

        s.read(stream, _parent, _root)?;

        Ok(s)
    }

    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S,
                             _parent: &Option<Box<KaitaiStruct>>,
                             _root: &Option<Box<KaitaiStruct>>)
                             -> Result<()>
        where Self: Sized {
        self.qty = stream.read_u4le()?;
        self.sizes = [];
        $n = $this->qty();
        for ($i = 0; $i < $n; $i++) {
            self.sizes[] = stream.read_u4le()?;
        }
        self.bufs = [];
        $n = $this->qty();
        for ($i = 0; $i < $n; $i++) {
            self.bufs[] = &mut S::bytesToStr(stream->readBytes($this->sizes()[$i]), "ASCII");
        }

        Ok(())
    }
}
