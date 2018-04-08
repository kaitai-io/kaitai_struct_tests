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

pub struct ExprIoPos {
pub struct AllPlusNumber {
    pub substream1: ,
    pub substream2: ,
    pub _raw_substream1: String,
    pub _raw_substream2: String,
    pub myStr: String,
    pub body: String,
    pub number: u16,
}

impl KaitaiStruct for ExprIoPos {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for AllPlusNumber {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
            substream1: ,
            substream2: ,
            _raw_substream1: String,
            _raw_substream2: String,
            myStr: String,
            body: String,
            number: 0,
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
        self._raw_substream1 = stream->readBytes(16);
        $io = new &mut S(self._raw_substream1);
        self.substream1 = new expr_io_pos::all_plus_number($io, $this, _root);
        self._raw_substream2 = stream->readBytes(14);
        $io = new &mut S(self._raw_substream2);
        self.substream2 = new expr_io_pos::all_plus_number($io, $this, _root);

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
        self.myStr = &mut S::bytesToStr(stream->readBytesTerm(0, false, true, true), "UTF-8");
        self.body = stream->readBytes((($this->_io()->size() - $this->_io()->pos()) - 2));
        self.number = stream.read_u2le()?;

        Ok(())
    }
}
