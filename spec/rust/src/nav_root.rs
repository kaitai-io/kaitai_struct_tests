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
pub struct NavRoot {
    pub header: Box<NavRoot__HeaderObj>,
    pub index: Box<NavRoot__IndexObj>,
}

impl KaitaiStruct for NavRoot {
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
        self.header = Box::new(NavRoot__HeaderObj::new(self.stream, self, _root)?);
        self.index = Box::new(NavRoot__IndexObj::new(self.stream, self, _root)?);
    }
}

impl NavRoot {
}
#[derive(Default)]
pub struct NavRoot__HeaderObj {
    pub qtyEntries: u32,
    pub filenameLen: u32,
}

impl KaitaiStruct for NavRoot__HeaderObj {
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
        self.qtyEntries = self.stream.read_u4le()?;
        self.filenameLen = self.stream.read_u4le()?;
    }
}

impl NavRoot__HeaderObj {
}
#[derive(Default)]
pub struct NavRoot__IndexObj {
    pub magic: Vec<u8>,
    pub entries: Vec<Box<NavRoot__Entry>>,
}

impl KaitaiStruct for NavRoot__IndexObj {
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
        self.magic = self.stream.read_bytes(4)?;
        self.entries = vec!();
        for i in 0..self._root.header.qty_entries {
            self.entries.push(Box::new(NavRoot__Entry::new(self.stream, self, _root)?));
        }
    }
}

impl NavRoot__IndexObj {
}
#[derive(Default)]
pub struct NavRoot__Entry {
    pub filename: String,
}

impl KaitaiStruct for NavRoot__Entry {
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
        self.filename = panic!("Unimplemented encoding for bytesToStr: {}", "UTF-8");
    }
}

impl NavRoot__Entry {
}
