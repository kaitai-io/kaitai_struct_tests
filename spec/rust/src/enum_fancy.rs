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

pub struct EnumFancy {
pub struct Animal {
    pub pet1: ,
    pub pet2: ,
}

impl KaitaiStruct for EnumFancy {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            pet1: ,
            pet2: ,
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
        self.pet1 = stream.read_u4le()?;
        self.pet2 = stream.read_u4le()?;

        Ok(())
    }
}

/**
 * A member of genus Canis.
 */
const DOG = 4;

/**
 * Small, typically furry, carnivorous mammal.
 */
const CAT = 7;
const CHICKEN = 12;
}
