// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

use std::{
    option::Option,
    boxed::Box,
    io::Result
};

use kaitai_struct::{
    KaitaiStream,
    KaitaiStruct
};

pub struct FixedStruct {
pub struct Header {
    pub hdr: ,
    pub magic1: String,
    pub uint8: u8,
    pub sint8: i8,
    pub magicUint: String,
    pub uint16: u16,
    pub uint32: u32,
    pub uint64: u64,
    pub magicSint: String,
    pub sint16: i16,
    pub sint32: i32,
    pub sint64: i64,
    pub magicUintLe: String,
    pub uint16le: u16,
    pub uint32le: u32,
    pub uint64le: u64,
    pub magicSintLe: String,
    pub sint16le: i16,
    pub sint32le: i32,
    pub sint64le: i64,
    pub magicUintBe: String,
    pub uint16be: u16,
    pub uint32be: u32,
    pub uint64be: u64,
    pub magicSintBe: String,
    pub sint16be: i16,
    pub sint32be: i32,
    pub sint64be: i64,
}

impl KaitaiStruct for FixedStruct {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for Header {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
            hdr: ,
            magic1: String,
            uint8: 0,
            sint8: 0,
            magicUint: String,
            uint16: 0,
            uint32: 0,
            uint64: 0,
            magicSint: String,
            sint16: 0,
            sint32: 0,
            sint64: 0,
            magicUintLe: String,
            uint16le: 0,
            uint32le: 0,
            uint64le: 0,
            magicSintLe: String,
            sint16le: 0,
            sint32le: 0,
            sint64le: 0,
            magicUintBe: String,
            uint16be: 0,
            uint32be: 0,
            uint64be: 0,
            magicSintBe: String,
            sint16be: 0,
            sint32be: 0,
            sint64be: 0,
        };

        s.read(stream, _parent, _root)?;

        Ok(s)
    }

    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S,
                             _parent: &Option<Box<KaitaiStruct>>,
                             _root: &Option<Box<KaitaiStruct>>)
                             -> Result<()>
        where Self: Sized {

        Ok(())
    }
    public function hdr() {
        if (self.hdr !== null)
            return self.hdr;
        $_pos = stream->pos();
        stream->seek(0);
        self.hdr = new fixed_struct::header(stream, $this, _root);
        stream->seek($_pos);
        return self.hdr;
    }
}
        };

        s.read(stream, _parent, _root)?;

        Ok(s)
    }

    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S,
                             _parent: &Option<Box<KaitaiStruct>>,
                             _root: &Option<Box<KaitaiStruct>>)
                             -> Result<()>
        where Self: Sized {
        self.magic1 = stream->ensureFixedContents("\x50\x41\x43\x4B\x2D\x31");
        self.uint8 = stream.read_u1()?;
        self.sint8 = stream.read_s1()?;
        self.magicUint = stream->ensureFixedContents("\x50\x41\x43\x4B\x2D\x55\x2D\x44\x45\x46");
        self.uint16 = stream.read_u2le()?;
        self.uint32 = stream.read_u4le()?;
        self.uint64 = stream.read_u8le()?;
        self.magicSint = stream->ensureFixedContents("\x50\x41\x43\x4B\x2D\x53\x2D\x44\x45\x46");
        self.sint16 = stream.read_s2le()?;
        self.sint32 = stream.read_s4le()?;
        self.sint64 = stream.read_s8le()?;
        self.magicUintLe = stream->ensureFixedContents("\x50\x41\x43\x4B\x2D\x55\x2D\x4C\x45");
        self.uint16le = stream.read_u2le()?;
        self.uint32le = stream.read_u4le()?;
        self.uint64le = stream.read_u8le()?;
        self.magicSintLe = stream->ensureFixedContents("\x50\x41\x43\x4B\x2D\x53\x2D\x4C\x45");
        self.sint16le = stream.read_s2le()?;
        self.sint32le = stream.read_s4le()?;
        self.sint64le = stream.read_s8le()?;
        self.magicUintBe = stream->ensureFixedContents("\x50\x41\x43\x4B\x2D\x55\x2D\x42\x45");
        self.uint16be = stream.read_u2be()?;
        self.uint32be = stream.read_u4be()?;
        self.uint64be = stream.read_u8be()?;
        self.magicSintBe = stream->ensureFixedContents("\x50\x41\x43\x4B\x2D\x53\x2D\x42\x45");
        self.sint16be = stream.read_s2be()?;
        self.sint32be = stream.read_s4be()?;
        self.sint64be = stream.read_s8be()?;

        Ok(())
    }
}
