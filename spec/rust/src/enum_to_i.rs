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

pub struct EnumToI {
pub struct Animal {
    pub pet1I: i32,
    pub pet1Mod: i32,
    pub oneLtTwo: bool,
    pub pet1: ,
    pub pet2: ,
}

impl KaitaiStruct for EnumToI {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            pet1I: i32,
            pet1Mod: i32,
            oneLtTwo: bool,
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
    public function pet1I() {
        if (self.pet1I !== null)
            return self.pet1I;
        self.pet1I = $this->pet1();
        return self.pet1I;
    }
    public function pet1Mod() {
        if (self.pet1Mod !== null)
            return self.pet1Mod;
        self.pet1Mod = ($this->pet1() + 32768);
        return self.pet1Mod;
    }
    public function oneLtTwo() {
        if (self.oneLtTwo !== null)
            return self.oneLtTwo;
        self.oneLtTwo = $this->pet1() < $this->pet2();
        return self.oneLtTwo;
    }
}
const DOG = 4;
const CAT = 7;
const CHICKEN = 12;
}
