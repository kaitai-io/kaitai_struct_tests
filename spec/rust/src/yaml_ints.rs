// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct YamlInts {
    pub testU4Dec: Option<i32>,
    pub testU4Hex: Option<i32>,
    pub testU8Dec: Option<i32>,
    pub testU8Hex: Option<i32>,
}

impl KaitaiStruct for YamlInts {
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
    }
}

impl YamlInts {
    fn testU4Dec(&mut self) -> i32 {
        if let Some(x) = self.testU4Dec {
            return x;
        }

        self.testU4Dec = 4294967295;
        return self.testU4Dec;
    }
    fn testU4Hex(&mut self) -> i32 {
        if let Some(x) = self.testU4Hex {
            return x;
        }

        self.testU4Hex = 4294967295;
        return self.testU4Hex;
    }
    fn testU8Dec(&mut self) -> i32 {
        if let Some(x) = self.testU8Dec {
            return x;
        }

        self.testU8Dec = 18446744073709551615;
        return self.testU8Dec;
    }
    fn testU8Hex(&mut self) -> i32 {
        if let Some(x) = self.testU8Hex {
            return x;
        }

        self.testU8Hex = 18446744073709551615;
        return self.testU8Hex;
    }
}
