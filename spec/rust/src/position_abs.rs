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

pub struct PositionAbs {
pub struct IndexObj {
    pub index: ,
    pub indexOffset: u32,
    pub entry: String,
}

impl KaitaiStruct for PositionAbs {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for IndexObj {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
            index: ,
            indexOffset: 0,
            entry: String,
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
        self.indexOffset = stream.read_u4le()?;

        Ok(())
    }
    public function index() {
        if (self.index !== null)
            return self.index;
        $_pos = stream->pos();
        stream->seek($this->indexOffset());
        self.index = new position_abs::index_obj(stream, $this, _root);
        stream->seek($_pos);
        return self.index;
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
        self.entry = &mut S::bytesToStr(stream->readBytesTerm(0, false, true, true), "UTF-8");

        Ok(())
    }
}
