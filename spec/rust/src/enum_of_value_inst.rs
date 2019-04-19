// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct EnumOfValueInst {
    pub pet1: Box<EnumOfValueInst__Animal>,
    pub pet2: Box<EnumOfValueInst__Animal>,
    pub pet3: Option<Box<EnumOfValueInst__Animal>>,
    pub pet4: Option<Box<EnumOfValueInst__Animal>>,
}

impl KaitaiStruct for EnumOfValueInst {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s: Self = Default::default();

        s.stream = stream;
        s.read(stream, _parent, _root)?;

        Ok(s)
    }


    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S,
                             _parent: &Option<Box<KaitaiStruct>>,
                             _root: &Option<Box<KaitaiStruct>>)
                             -> Result<()>
        where Self: Sized {
        self.pet1 = self.stream.read_u4le()?;
        self.pet2 = self.stream.read_u4le()?;
    }
}

impl EnumOfValueInst {
    fn pet3(&mut self) -> Box<EnumOfValueInst__Animal> {
        if let Some(x) = self.pet3 {
            return x;
        }

        self.pet3 = if self.pet_1 == EnumOfValueInst__Animal::CAT { 4 } else { 12};
        return self.pet3;
    }
    fn pet4(&mut self) -> Box<EnumOfValueInst__Animal> {
        if let Some(x) = self.pet4 {
            return x;
        }

        self.pet4 = if self.pet_1 == EnumOfValueInst__Animal::CAT { EnumOfValueInst__Animal::DOG } else { EnumOfValueInst__Animal::CHICKEN};
        return self.pet4;
    }
}
enum EnumOfValueInst__Animal {
    DOG,
    CAT,
    CHICKEN,
}
