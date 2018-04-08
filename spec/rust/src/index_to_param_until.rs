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

pub struct IndexToParamUntil {
pub struct Block {
    pub qty: u32,
    pub sizes: Vec<u32>*,
    pub blocks: Vec<>*,
    pub buf: String,
    pub idx: i32,
}

impl KaitaiStruct for IndexToParamUntil {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for Block {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
            qty: 0,
            sizes: Vec<u32>*,
            blocks: Vec<>*,
            buf: String,
            idx: 0,
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
        self.blocks = [];
        $i = 0;
        do {
            $_ = new index_to_param_until::block($i, stream, $this, _root);
            self.blocks[] = $_;
            $i++;
        } while (!($this->_io()->isEof()));

        Ok(())
    }
}
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
        self.buf = &mut S::bytesToStr(stream->readBytes($this->_root()->sizes()[$this->idx()]), "ASCII");

        Ok(())
    }
}
