// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct FixedStruct {
    pub hdr: Option<Box<FixedStruct__Header>>,
}

impl KaitaiStruct for FixedStruct {
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

impl FixedStruct {
    fn hdr(&mut self) -> Box<FixedStruct__Header> {
        if let Some(x) = self.hdr {
            return x;
        }

        let _pos = self.stream.pos();
        self.stream.seek(0);
        self.hdr = Box::new(FixedStruct__Header::new(self.stream, self, _root)?);
        self.stream.seek(_pos);
        return self.hdr;
    }
}
#[derive(Default)]
pub struct FixedStruct__Header {
    pub magic1: Vec<u8>,
    pub uint8: u8,
    pub sint8: i8,
    pub magicUint: Vec<u8>,
    pub uint16: u16,
    pub uint32: u32,
    pub uint64: u64,
    pub magicSint: Vec<u8>,
    pub sint16: i16,
    pub sint32: i32,
    pub sint64: i64,
    pub magicUintLe: Vec<u8>,
    pub uint16le: u16,
    pub uint32le: u32,
    pub uint64le: u64,
    pub magicSintLe: Vec<u8>,
    pub sint16le: i16,
    pub sint32le: i32,
    pub sint64le: i64,
    pub magicUintBe: Vec<u8>,
    pub uint16be: u16,
    pub uint32be: u32,
    pub uint64be: u64,
    pub magicSintBe: Vec<u8>,
    pub sint16be: i16,
    pub sint32be: i32,
    pub sint64be: i64,
}

impl KaitaiStruct for FixedStruct__Header {
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
        self.magic1 = self.stream.ensureFixedContents(vec!([0x50, 0x41, 0x43, 0x4b, 0x2d, 0x31]));
        self.uint8 = self.stream.read_u1()?;
        self.sint8 = self.stream.read_s1()?;
        self.magicUint = self.stream.ensureFixedContents(vec!([0x50, 0x41, 0x43, 0x4b, 0x2d, 0x55, 0x2d, 0x44, 0x45, 0x46]));
        self.uint16 = self.stream.read_u2le()?;
        self.uint32 = self.stream.read_u4le()?;
        self.uint64 = self.stream.read_u8le()?;
        self.magicSint = self.stream.ensureFixedContents(vec!([0x50, 0x41, 0x43, 0x4b, 0x2d, 0x53, 0x2d, 0x44, 0x45, 0x46]));
        self.sint16 = self.stream.read_s2le()?;
        self.sint32 = self.stream.read_s4le()?;
        self.sint64 = self.stream.read_s8le()?;
        self.magicUintLe = self.stream.ensureFixedContents(vec!([0x50, 0x41, 0x43, 0x4b, 0x2d, 0x55, 0x2d, 0x4c, 0x45]));
        self.uint16le = self.stream.read_u2le()?;
        self.uint32le = self.stream.read_u4le()?;
        self.uint64le = self.stream.read_u8le()?;
        self.magicSintLe = self.stream.ensureFixedContents(vec!([0x50, 0x41, 0x43, 0x4b, 0x2d, 0x53, 0x2d, 0x4c, 0x45]));
        self.sint16le = self.stream.read_s2le()?;
        self.sint32le = self.stream.read_s4le()?;
        self.sint64le = self.stream.read_s8le()?;
        self.magicUintBe = self.stream.ensureFixedContents(vec!([0x50, 0x41, 0x43, 0x4b, 0x2d, 0x55, 0x2d, 0x42, 0x45]));
        self.uint16be = self.stream.read_u2be()?;
        self.uint32be = self.stream.read_u4be()?;
        self.uint64be = self.stream.read_u8be()?;
        self.magicSintBe = self.stream.ensureFixedContents(vec!([0x50, 0x41, 0x43, 0x4b, 0x2d, 0x53, 0x2d, 0x42, 0x45]));
        self.sint16be = self.stream.read_s2be()?;
        self.sint32be = self.stream.read_s4be()?;
        self.sint64be = self.stream.read_s8be()?;
    }
}

impl FixedStruct__Header {
}
