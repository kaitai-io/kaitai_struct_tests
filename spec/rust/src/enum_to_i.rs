// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::io::Cursor;
use std::vec::Vec;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct EnumToI {
    pub pet1: Box<EnumToI__Animal>,
    pub pet2: Box<EnumToI__Animal>,
    pub pet1I: Option<i32>,
    pub pet1Mod: Option<i32>,
    pub oneLtTwo: Option<bool>,
}

impl KaitaiStruct for EnumToI {
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

impl EnumToI {
    fn pet1I(&mut self) -> i32 {
        if let Some(x) = self.pet1I {
            return x;
        }

        self.pet1I = self.pet_1;
        return self.pet1I;
    }
    fn pet1Mod(&mut self) -> i32 {
        if let Some(x) = self.pet1Mod {
            return x;
        }

        self.pet1Mod = (self.pet_1 + 32768);
        return self.pet1Mod;
    }
    fn oneLtTwo(&mut self) -> bool {
        if let Some(x) = self.oneLtTwo {
            return x;
        }

        self.oneLtTwo = self.pet_1 < self.pet_2;
        return self.oneLtTwo;
    }
}
enum EnumToI__Animal {
    DOG,
    CAT,
    CHICKEN,
}
