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

pub struct Debug0 {
    pub one: u8,
    pub arrayOfInts: Vec<u8>*,
    pub _unnamed2: u8,
}

impl KaitaiStruct for Debug0 {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            one: 0,
            arrayOfInts: Vec<u8>*,
            _unnamed2: 0,
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
        self.one = stream.read_u1()?;
        self.arrayOfInts = [];
        $n = 3;
        for ($i = 0; $i < $n; $i++) {
            self.arrayOfInts[] = stream.read_u1()?;
        }
        self._unnamed2 = stream.read_u1()?;

        Ok(())
    }
}
