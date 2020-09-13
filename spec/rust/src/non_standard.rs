// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct NonStandard {
    pub foo: u8,
    pub bar: u32,
    pub vi: Option<u8>,
    pub pi: Option<u8>,
}

impl KaitaiStruct for NonStandard {
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
        self.foo = self.stream.read_u1()?;
        match self.foo {
            42 => {
                self.bar = self.stream.read_u2le()?;
            },
            43 => {
                self.bar = self.stream.read_u4le()?;
            },
        }
    }
}

impl NonStandard {
    fn vi(&mut self) -> u8 {
        if let Some(x) = self.vi {
            return x;
        }

        self.vi = self.foo;
        return self.vi;
    }
    fn pi(&mut self) -> u8 {
        if let Some(x) = self.pi {
            return x;
        }

        let _pos = self.stream.pos();
        self.stream.seek(0);
        self.pi = self.stream.read_u1()?;
        self.stream.seek(_pos);
        return self.pi;
    }
}
