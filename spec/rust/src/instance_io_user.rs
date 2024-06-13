// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;
use std::vec::Vec;
use std::io::Cursor;

#[derive(Default)]
pub struct InstanceIoUser {
    pub qtyEntries: u32,
    pub entries: Vec<Box<InstanceIoUser__Entry>>,
    pub strings: Box<InstanceIoUser__StringsObj>,
    pub _raw_strings: Vec<u8>,
}

impl KaitaiStruct for InstanceIoUser {
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
        self.entries = vec!();
        for i in 0..self.qty_entries {
            self.entries.push(Box::new(InstanceIoUser__Entry::new(self.stream, self, _root)?));
        }
        self._raw_strings = self.stream.read_bytes_full()?;
        let mut io = Cursor::new(self._raw_strings);
        self.strings = Box::new(InstanceIoUser__StringsObj::new(self.stream, self, _root)?);
    }
}

impl InstanceIoUser {
}
#[derive(Default)]
pub struct InstanceIoUser__Entry {
    pub nameOfs: u32,
    pub value: u32,
    pub name: Option<String>,
}

impl KaitaiStruct for InstanceIoUser__Entry {
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
        self.nameOfs = self.stream.read_u4le()?;
        self.value = self.stream.read_u4le()?;
    }
}

impl InstanceIoUser__Entry {
    fn name(&mut self) -> String {
        if let Some(x) = self.name {
            return x;
        }

        let mut io = self._root.strings._io;
        let _pos = io.pos();
        io.seek(self.name_ofs);
        self.name = panic!("Unimplemented encoding for bytesToStr: {}", "UTF-8");
        io.seek(_pos);
        return self.name;
    }
}
#[derive(Default)]
pub struct InstanceIoUser__StringsObj {
    pub str: Vec<String>,
}

impl KaitaiStruct for InstanceIoUser__StringsObj {
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
        self.str = [];
        while !self.stream.isEof() {
            self.str.push(panic!("Unimplemented encoding for bytesToStr: {}", "UTF-8"));
        }
    }
}

impl InstanceIoUser__StringsObj {
}
