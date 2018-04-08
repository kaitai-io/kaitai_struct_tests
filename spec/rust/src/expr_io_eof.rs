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

pub struct ExprIoEof {
pub struct OneOrTwo {
    pub substream1: ,
    pub substream2: ,
    pub _raw_substream1: String,
    pub _raw_substream2: String,
    pub reflectEof: bool,
    pub one: u32,
    pub two: u32,
}

impl KaitaiStruct for ExprIoEof {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for OneOrTwo {
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
            reflectEof: bool,
            one: 0,
            two: 0,
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
        self._raw_substream1 = stream->readBytes(4);
        $io = new &mut S(self._raw_substream1);
        self.substream1 = new expr_io_eof::one_or_two($io, $this, _root);
        self._raw_substream2 = stream->readBytes(8);
        $io = new &mut S(self._raw_substream2);
        self.substream2 = new expr_io_eof::one_or_two($io, $this, _root);

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
        self.one = stream.read_u4le()?;
        if (!($this->_io()->isEof())) {
            self.two = stream.read_u4le()?;
        }

        Ok(())
    }
    public function reflectEof() {
        if (self.reflectEof !== null)
            return self.reflectEof;
        self.reflectEof = $this->_io()->isEof();
        return self.reflectEof;
    }
}
