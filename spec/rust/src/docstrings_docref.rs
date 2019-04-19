// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;


/*
 * Another one-liner
 */
#[derive(Default)]
pub struct DocstringsDocref {
    pub one: u8,
    pub two: u8,
    pub three: u8,
    pub foo: Option<bool>,
    pub parseInst: Option<u8>,
}

impl KaitaiStruct for DocstringsDocref {
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
        self.two = self.stream.read_u1()?;
        self.three = self.stream.read_u1()?;
    }
}

impl DocstringsDocref {
    fn foo(&mut self) -> bool {
        if let Some(x) = self.foo {
            return x;
        }

        self.foo = true;
        return self.foo;
    }
    fn parseInst(&mut self) -> u8 {
        if let Some(x) = self.parseInst {
            return x;
        }

        let _pos = self.stream.pos();
        self.stream.seek(0);
        self.parseInst = self.stream.read_u1()?;
        self.stream.seek(_pos);
        return self.parseInst;
    }

    /*
     * Both doc and doc-ref are defined
     */
}
