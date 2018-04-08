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

pub struct OptionalId {
    pub _unnamed0: u8,
    pub _unnamed1: u8,
    pub _unnamed2: String,
}

impl KaitaiStruct for OptionalId {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            _unnamed0: 0,
            _unnamed1: 0,
            _unnamed2: String,
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
        self._unnamed0 = stream.read_u1()?;
        self._unnamed1 = stream.read_u1()?;
        self._unnamed2 = stream->readBytes(5);

        Ok(())
    }
}
