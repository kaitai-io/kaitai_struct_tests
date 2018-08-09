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
pub struct MultipleUse {
    pub t1: Box<MultipleUse__Type1>,
    pub t2: Box<MultipleUse__Type2>,
}

impl KaitaiStruct for MultipleUse {
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
        self.t1 = Box::new(MultipleUse__Type1::new(self.stream, self, _root)?);
        self.t2 = Box::new(MultipleUse__Type2::new(self.stream, self, _root)?);
    }
}

impl MultipleUse {
}
#[derive(Default)]
pub struct MultipleUse__Multi {
    pub value: i32,
}

impl KaitaiStruct for MultipleUse__Multi {
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
        self.value = self.stream.read_s4le()?;
    }
}

impl MultipleUse__Multi {
}
#[derive(Default)]
pub struct MultipleUse__Type1 {
    pub firstUse: Box<MultipleUse__Multi>,
}

impl KaitaiStruct for MultipleUse__Type1 {
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
        self.firstUse = Box::new(MultipleUse__Multi::new(self.stream, self, _root)?);
    }
}

impl MultipleUse__Type1 {
}
#[derive(Default)]
pub struct MultipleUse__Type2 {
    pub secondUse: Option<Box<MultipleUse__Multi>>,
}

impl KaitaiStruct for MultipleUse__Type2 {
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

impl MultipleUse__Type2 {
    fn secondUse(&mut self) -> Box<MultipleUse__Multi> {
        if let Some(x) = self.secondUse {
            return x;
        }

        let _pos = self.stream.pos();
        self.stream.seek(0);
        self.secondUse = Box::new(MultipleUse__Multi::new(self.stream, self, _root)?);
        self.stream.seek(_pos);
        return self.secondUse;
    }
}
