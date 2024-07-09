// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct ExprEnum {
    pub one: u8,
    pub constDog: Option<Box<ExprEnum__Animal>>,
    pub derivedBoom: Option<Box<ExprEnum__Animal>>,
    pub derivedDog: Option<Box<ExprEnum__Animal>>,
}

impl KaitaiStruct for ExprEnum {
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
        self.one = self.stream.read_u1()?;
    }
}

impl ExprEnum {
    fn constDog(&mut self) -> Box<ExprEnum__Animal> {
        if let Some(x) = self.constDog {
            return x;
        }

        self.constDog = 4;
        return self.constDog;
    }
    fn derivedBoom(&mut self) -> Box<ExprEnum__Animal> {
        if let Some(x) = self.derivedBoom {
            return x;
        }

        self.derivedBoom = self.one;
        return self.derivedBoom;
    }
    fn derivedDog(&mut self) -> Box<ExprEnum__Animal> {
        if let Some(x) = self.derivedDog {
            return x;
        }

        self.derivedDog = (self.one - 98);
        return self.derivedDog;
    }
}
enum ExprEnum__Animal {
    DOG,
    CAT,
    CHICKEN,
    BOOM,
}
