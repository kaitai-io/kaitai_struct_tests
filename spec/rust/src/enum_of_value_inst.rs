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

pub struct EnumOfValueInst {
pub struct Animal {
    pub pet3: ,
    pub pet4: ,
    pub pet1: ,
    pub pet2: ,
}

impl KaitaiStruct for EnumOfValueInst {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            pet3: ,
            pet4: ,
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
    public function pet3() {
        if (self.pet3 !== null)
            return self.pet3;
        self.pet3 = ($this->pet1() == enum_of_value_inst::animal::CAT ? 4 : 12);
        return self.pet3;
    }
    public function pet4() {
        if (self.pet4 !== null)
            return self.pet4;
        self.pet4 = ($this->pet1() == enum_of_value_inst::animal::CAT ? enum_of_value_inst::animal::DOG : enum_of_value_inst::animal::CHICKEN);
        return self.pet4;
    }
}
const DOG = 4;
const CAT = 7;
const CHICKEN = 12;
}
