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
pub struct InstanceUserArray {
    pub ofs: u32,
    pub entrySize: u32,
    pub qtyEntries: u32,
    pub userEntries: Option<Vec<Box<InstanceUserArray__Entry>>>,
}

impl KaitaiStruct for InstanceUserArray {
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
        self.ofs = self.stream.read_u4le()?;
        self.entrySize = self.stream.read_u4le()?;
        self.qtyEntries = self.stream.read_u4le()?;
    }
}

impl InstanceUserArray {
    fn userEntries(&mut self) -> Vec<Box<InstanceUserArray__Entry>> {
        if let Some(x) = self.userEntries {
            return x;
        }

        if self.ofs > 0 {
            let _pos = self.stream.pos();
            self.stream.seek(self.ofs);
            self._raw_userEntries = vec!();
            self.userEntries = vec!();
            for i in 0..self.qty_entries {
                self._raw_userEntries.push(self.stream.read_bytes(self.entry_size)?);
                let mut io = Cursor::new(self._raw_userEntries.last());
                self.userEntries.push(Box::new(InstanceUserArray__Entry::new(self.stream, self, _root)?));
            }
            self.stream.seek(_pos);
        }
        return self.userEntries;
    }
}
#[derive(Default)]
pub struct InstanceUserArray__Entry {
    pub word1: u16,
    pub word2: u16,
}

impl KaitaiStruct for InstanceUserArray__Entry {
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
        self.word1 = self.stream.read_u2le()?;
        self.word2 = self.stream.read_u2le()?;
    }
}

impl InstanceUserArray__Entry {
}
