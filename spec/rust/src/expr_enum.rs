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

pub struct ExprEnum {
pub struct Animal {
    pub constDog: ,
    pub derivedBoom: ,
    pub derivedDog: ,
    pub one: u8,
}

impl KaitaiStruct for ExprEnum {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            constDog: ,
            derivedBoom: ,
            derivedDog: ,
            one: 0,
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

        Ok(())
    }
    public function constDog() {
        if (self.constDog !== null)
            return self.constDog;
        self.constDog = 4;
        return self.constDog;
    }
    public function derivedBoom() {
        if (self.derivedBoom !== null)
            return self.derivedBoom;
        self.derivedBoom = $this->one();
        return self.derivedBoom;
    }
    public function derivedDog() {
        if (self.derivedDog !== null)
            return self.derivedDog;
        self.derivedDog = ($this->one() - 98);
        return self.derivedDog;
    }
}
const DOG = 4;
const CAT = 7;
const CHICKEN = 12;
const BOOM = 102;
}
