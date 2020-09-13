// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct PositionToEnd {
    pub index: Option<Box<PositionToEnd__IndexObj>>,
}

impl KaitaiStruct for PositionToEnd {
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

impl PositionToEnd {
    fn index(&mut self) -> Box<PositionToEnd__IndexObj> {
        if let Some(x) = self.index {
            return x;
        }

        let _pos = self.stream.pos();
        self.stream.seek((self._io.size - 8));
        self.index = Box::new(PositionToEnd__IndexObj::new(self.stream, self, _root)?);
        self.stream.seek(_pos);
        return self.index;
    }
}
#[derive(Default)]
pub struct PositionToEnd__IndexObj {
    pub foo: u32,
    pub bar: u32,
}

impl KaitaiStruct for PositionToEnd__IndexObj {
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
        self.foo = self.stream.read_u4le()?;
        self.bar = self.stream.read_u4le()?;
    }
}

impl PositionToEnd__IndexObj {
}
