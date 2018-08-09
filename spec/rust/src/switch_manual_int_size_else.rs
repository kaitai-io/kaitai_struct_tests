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
pub struct SwitchManualIntSizeElse {
    pub chunks: Vec<Box<SwitchManualIntSizeElse__Chunk>>,
}

impl KaitaiStruct for SwitchManualIntSizeElse {
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
        self.chunks = [];
        while !self.stream.isEof() {
            self.chunks.push(Box::new(SwitchManualIntSizeElse__Chunk::new(self.stream, self, _root)?));
        }
    }
}

impl SwitchManualIntSizeElse {
}
#[derive(Default)]
pub struct SwitchManualIntSizeElse__Chunk {
    pub code: u8,
    pub size: u32,
    pub body: Option<Box<KaitaiStruct>>,
}

impl KaitaiStruct for SwitchManualIntSizeElse__Chunk {
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
        self.code = self.stream.read_u1()?;
        self.size = self.stream.read_u4le()?;
        match self.code {
            17 => {
                self._raw_body = self.stream.read_bytes(self.size)?;
                let mut io = Cursor::new(self._raw_body);
                self.body = Box::new(SwitchManualIntSizeElse__Chunk__ChunkMeta::new(self.stream, self, _root)?);
            },
            34 => {
                self._raw_body = self.stream.read_bytes(self.size)?;
                let mut io = Cursor::new(self._raw_body);
                self.body = Box::new(SwitchManualIntSizeElse__Chunk__ChunkDir::new(self.stream, self, _root)?);
            },
            _ => {
                self._raw_body = self.stream.read_bytes(self.size)?;
                let mut io = Cursor::new(self._raw_body);
                self.body = Box::new(SwitchManualIntSizeElse__Chunk__Dummy::new(self.stream, self, _root)?);
            }
        }
    }
}

impl SwitchManualIntSizeElse__Chunk {
}
#[derive(Default)]
pub struct SwitchManualIntSizeElse__Chunk__ChunkMeta {
    pub title: String,
    pub author: String,
}

impl KaitaiStruct for SwitchManualIntSizeElse__Chunk__ChunkMeta {
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
        self.title = panic!("Unimplemented encoding for bytesToStr: {}", "UTF-8");
        self.author = panic!("Unimplemented encoding for bytesToStr: {}", "UTF-8");
    }
}

impl SwitchManualIntSizeElse__Chunk__ChunkMeta {
}
#[derive(Default)]
pub struct SwitchManualIntSizeElse__Chunk__ChunkDir {
    pub entries: Vec<String>,
}

impl KaitaiStruct for SwitchManualIntSizeElse__Chunk__ChunkDir {
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
        self.entries = [];
        while !self.stream.isEof() {
            self.entries.push(panic!("Unimplemented encoding for bytesToStr: {}", "UTF-8"));
        }
    }
}

impl SwitchManualIntSizeElse__Chunk__ChunkDir {
}
#[derive(Default)]
pub struct SwitchManualIntSizeElse__Chunk__Dummy {
    pub rest: Vec<u8>,
}

impl KaitaiStruct for SwitchManualIntSizeElse__Chunk__Dummy {
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
        self.rest = self.stream.read_bytes_full()?;
    }
}

impl SwitchManualIntSizeElse__Chunk__Dummy {
}
