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

pub struct BufferedStruct {
pub struct Block {
    pub len1: u32,
    pub block1: ,
    pub len2: u32,
    pub block2: ,
    pub finisher: u32,
    pub _raw_block1: String,
    pub _raw_block2: String,
    pub number1: u32,
    pub number2: u32,
}

impl KaitaiStruct for BufferedStruct {
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
            len1: 0,
            block1: ,
            len2: 0,
            block2: ,
            finisher: 0,
            _raw_block1: String,
            _raw_block2: String,
            number1: 0,
            number2: 0,
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
        self.len1 = stream.read_u4le()?;
        self._raw_block1 = stream->readBytes($this->len1());
        $io = new &mut S(self._raw_block1);
        self.block1 = new buffered_struct::block($io, $this, _root);
        self.len2 = stream.read_u4le()?;
        self._raw_block2 = stream->readBytes($this->len2());
        $io = new &mut S(self._raw_block2);
        self.block2 = new buffered_struct::block($io, $this, _root);
        self.finisher = stream.read_u4le()?;

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
        self.number1 = stream.read_u4le()?;
        self.number2 = stream.read_u4le()?;

        Ok(())
    }
}
