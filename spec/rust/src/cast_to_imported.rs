// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;
use hello_world::HelloWorld;

#[derive(Default)]
pub struct CastToImported {
    pub one: Box<HelloWorld>,
    pub oneCasted: Option<Box<HelloWorld>>,
}

impl KaitaiStruct for CastToImported {
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
        self.one = Box::new(HelloWorld::new(self.stream, self, _root)?);
    }
}

impl CastToImported {
    fn oneCasted(&mut self) -> Box<HelloWorld> {
        if let Some(x) = self.oneCasted {
            return x;
        }

        self.oneCasted = self.one;
        return self.oneCasted;
    }
}
