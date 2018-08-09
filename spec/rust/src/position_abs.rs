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
pub struct PositionAbs {
    pub indexOffset: u32,
    pub index: Option<Box<PositionAbs__IndexObj>>,
}

impl KaitaiStruct for PositionAbs {
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
        self.indexOffset = self.stream.read_u4le()?;
    }
}

impl PositionAbs {
    fn index(&mut self) -> Box<PositionAbs__IndexObj> {
        if let Some(x) = self.index {
            return x;
        }

        let _pos = self.stream.pos();
        self.stream.seek(self.index_offset);
        self.index = Box::new(PositionAbs__IndexObj::new(self.stream, self, _root)?);
        self.stream.seek(_pos);
        return self.index;
    }
}
#[derive(Default)]
pub struct PositionAbs__IndexObj {
    pub entry: String,
}

impl KaitaiStruct for PositionAbs__IndexObj {
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
        self.entry = panic!("Unimplemented encoding for bytesToStr: {}", "UTF-8");
    }
}

impl PositionAbs__IndexObj {
}
