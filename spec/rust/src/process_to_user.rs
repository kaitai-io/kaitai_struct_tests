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

pub struct ProcessToUser {
pub struct JustStr {
    pub buf1: ,
    pub _raw__raw_buf1: String,
    pub _raw_buf1: String,
    pub str: String,
}

impl KaitaiStruct for ProcessToUser {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for JustStr {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
            buf1: ,
            _raw__raw_buf1: String,
            _raw_buf1: String,
            str: String,
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
        self._raw__raw_buf1 = stream->readBytes(5);
        self._raw_buf1 = &mut S::processRotateLeft(self._raw__raw_buf1, 3, 1);
        $io = new &mut S(self._raw_buf1);
        self.buf1 = new process_to_user::just_str($io, $this, _root);

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
        self.str = &mut S::bytesToStr(stream->readBytesFull(), "UTF-8");

        Ok(())
    }
}
