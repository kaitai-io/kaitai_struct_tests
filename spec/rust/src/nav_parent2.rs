// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;
use std::vec::Vec;

#[derive(Default)]
pub struct NavParent2 {
    pub ofsTags: u32,
    pub numTags: u32,
    pub tags: Vec<Box<NavParent2__Tag>>,
}

impl KaitaiStruct for NavParent2 {
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
        self.ofsTags = self.stream.read_u4le()?;
        self.numTags = self.stream.read_u4le()?;
        self.tags = vec!();
        for i in 0..self.num_tags {
            self.tags.push(Box::new(NavParent2__Tag::new(self.stream, self, _root)?));
        }
    }
}

impl NavParent2 {
}
#[derive(Default)]
pub struct NavParent2__Tag {
    pub name: String,
    pub ofs: u32,
    pub numItems: u32,
    pub tagContent: Option<Box<NavParent2__Tag__TagChar>>,
}

impl KaitaiStruct for NavParent2__Tag {
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
        self.name = String::from_utf8_lossy(self.stream.read_bytes(4)?);
        self.ofs = self.stream.read_u4le()?;
        self.numItems = self.stream.read_u4le()?;
    }
}

impl NavParent2__Tag {
    fn tagContent(&mut self) -> Box<NavParent2__Tag__TagChar> {
        if let Some(x) = self.tagContent {
            return x;
        }

        let mut io = self._root._io;
        let _pos = io.pos();
        io.seek(self.ofs);
        match self.name {
            "RAHC" => {
                self.tagContent = Box::new(NavParent2__Tag__TagChar::new(self.stream, self, _root)?);
            },
        }
        io.seek(_pos);
        return self.tagContent;
    }
}
#[derive(Default)]
pub struct NavParent2__Tag__TagChar {
    pub content: String,
}

impl KaitaiStruct for NavParent2__Tag__TagChar {
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
        self.content = String::from_utf8_lossy(self.stream.read_bytes(self._parent.num_items)?);
    }
}

impl NavParent2__Tag__TagChar {
}
