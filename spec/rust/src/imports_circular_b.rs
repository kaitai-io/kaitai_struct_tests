// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::io::Cursor;
use std::vec::Vec;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;
use imports_circular_a::ImportsCircularA;

#[derive(Default)]
pub struct ImportsCircularB {
    pub initial: u8,
    pub backRef: Box<ImportsCircularA>,
}

impl KaitaiStruct for ImportsCircularB {
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
        self.initial = self.stream.read_u1()?;
        if self.initial == 65 {
            self.backRef = Box::new(ImportsCircularA::new(self.stream, self, _root)?);
        }
    }
}

impl ImportsCircularB {
}
