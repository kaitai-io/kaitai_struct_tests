// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::io::Cursor;
use std::vec::Vec;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;
use hello_world::HelloWorld;

#[derive(Default)]
pub struct Imports0 {
    pub two: u8,
    pub hw: Box<HelloWorld>,
    pub hwOne: Option<u8>,
}

impl KaitaiStruct for Imports0 {
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
        self.two = self.stream.read_u1()?;
        self.hw = Box::new(HelloWorld::new(self.stream, self, _root)?);
    }
}

impl Imports0 {
    fn hwOne(&mut self) -> u8 {
        if let Some(x) = self.hwOne {
            return x;
        }

        self.hwOne = self.hw.one;
        return self.hwOne;
    }
}
