// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;
use std::io::Cursor;

#[derive(Default)]
pub struct TypeTernary {
    pub difWoHack: Box<TypeTernary__Dummy>,
    pub difWithHack: Box<TypeTernary__Dummy>,
    pub _raw_difWoHack: Vec<u8>,
    pub _raw_difWithHack: Vec<u8>,
    pub _raw__raw_difWithHack: Vec<u8>,
    pub isHack: Option<bool>,
    pub dif: Option<Box<TypeTernary__Dummy>>,
    pub difValue: Option<u8>,
}

impl KaitaiStruct for TypeTernary {
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
        if !(self.is_hack) {
            self._raw_difWoHack = self.stream.read_bytes(1)?;
            let mut io = Cursor::new(self._raw_difWoHack);
            self.difWoHack = Box::new(TypeTernary__Dummy::new(self.stream, self, _root)?);
        }
        self._raw__raw_difWithHack = self.stream.read_bytes(1)?;
        self._raw_difWithHack = &mut S::processXorOne(self._raw__raw_difWithHack, 3);
        let mut io = Cursor::new(self._raw_difWithHack);
        self.difWithHack = Box::new(TypeTernary__Dummy::new(self.stream, self, _root)?);
    }
}

impl TypeTernary {
    fn isHack(&mut self) -> bool {
        if let Some(x) = self.isHack {
            return x;
        }

        self.isHack = true;
        return self.isHack;
    }
    fn dif(&mut self) -> Box<TypeTernary__Dummy> {
        if let Some(x) = self.dif {
            return x;
        }

        self.dif = if !(self.is_hack) { self.dif_wo_hack } else { self.dif_with_hack};
        return self.dif;
    }
    fn difValue(&mut self) -> u8 {
        if let Some(x) = self.difValue {
            return x;
        }

        self.difValue = self.dif.value;
        return self.difValue;
    }
}
#[derive(Default)]
pub struct TypeTernary__Dummy {
    pub value: u8,
}

impl KaitaiStruct for TypeTernary__Dummy {
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
        self.value = self.stream.read_u1()?;
    }
}

impl TypeTernary__Dummy {
}
