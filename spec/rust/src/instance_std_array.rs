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
pub struct InstanceStdArray {
    pub ofs: u32,
    pub entrySize: u32,
    pub qtyEntries: u32,
    pub entries: Option<Vec<Vec<u8>>>,
}

impl KaitaiStruct for InstanceStdArray {
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

impl InstanceStdArray {
    fn entries(&mut self) -> Vec<Vec<u8>> {
        if let Some(x) = self.entries {
            return x;
        }

        let _pos = self.stream.pos();
        self.stream.seek(self.ofs);
        self.entries = vec!();
        for i in 0..self.qty_entries {
            self.entries.push(self.stream.read_bytes(self.entry_size)?);
        }
        self.stream.seek(_pos);
        return self.entries;
    }
}
