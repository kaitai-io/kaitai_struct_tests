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
pub struct Expr2 {
    pub str1: Box<Expr2__ModStr>,
    pub str2: Box<Expr2__ModStr>,
    pub str1LenMod: Option<i32>,
    pub str1Len: Option<i32>,
    pub str1Tuple5: Option<Box<Expr2__Tuple>>,
    pub str2Tuple5: Option<Box<Expr2__Tuple>>,
    pub str1Avg: Option<i32>,
    pub str1Byte1: Option<u8>,
    pub str1Char5: Option<String>,
}

impl KaitaiStruct for Expr2 {
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
        self.str1 = Box::new(Expr2__ModStr::new(self.stream, self, _root)?);
        self.str2 = Box::new(Expr2__ModStr::new(self.stream, self, _root)?);
    }
}

impl Expr2 {
    fn str1LenMod(&mut self) -> i32 {
        if let Some(x) = self.str1LenMod {
            return x;
        }

        self.str1LenMod = self.str1.len_mod;
        return self.str1LenMod;
    }
    fn str1Len(&mut self) -> i32 {
        if let Some(x) = self.str1Len {
            return x;
        }

        self.str1Len = self.str1.str.len();
        return self.str1Len;
    }
    fn str1Tuple5(&mut self) -> Box<Expr2__Tuple> {
        if let Some(x) = self.str1Tuple5 {
            return x;
        }

        self.str1Tuple5 = self.str1.tuple5;
        return self.str1Tuple5;
    }
    fn str2Tuple5(&mut self) -> Box<Expr2__Tuple> {
        if let Some(x) = self.str2Tuple5 {
            return x;
        }

        self.str2Tuple5 = self.str2.tuple5;
        return self.str2Tuple5;
    }
    fn str1Avg(&mut self) -> i32 {
        if let Some(x) = self.str1Avg {
            return x;
        }

        self.str1Avg = self.str1.rest.avg;
        return self.str1Avg;
    }
    fn str1Byte1(&mut self) -> u8 {
        if let Some(x) = self.str1Byte1 {
            return x;
        }

        self.str1Byte1 = self.str1.rest.byte1;
        return self.str1Byte1;
    }
    fn str1Char5(&mut self) -> String {
        if let Some(x) = self.str1Char5 {
            return x;
        }

        self.str1Char5 = self.str1.char5;
        return self.str1Char5;
    }
}
#[derive(Default)]
pub struct Expr2__ModStr {
    pub lenOrig: u16,
    pub str: String,
    pub rest: Box<Expr2__Tuple>,
    pub _raw_rest: Vec<u8>,
    pub lenMod: Option<i32>,
    pub char5: Option<String>,
    pub tuple5: Option<Box<Expr2__Tuple>>,
}

impl KaitaiStruct for Expr2__ModStr {
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
        self.lenOrig = self.stream.read_u2le()?;
        self.str = panic!("Unimplemented encoding for bytesToStr: {}", "UTF-8");
        self._raw_rest = self.stream.read_bytes(3)?;
        let mut io = Cursor::new(self._raw_rest);
        self.rest = Box::new(Expr2__Tuple::new(self.stream, self, _root)?);
    }
}

impl Expr2__ModStr {
    fn lenMod(&mut self) -> i32 {
        if let Some(x) = self.lenMod {
            return x;
        }

        self.lenMod = (self.len_orig - 3);
        return self.lenMod;
    }
    fn char5(&mut self) -> String {
        if let Some(x) = self.char5 {
            return x;
        }

        let _pos = self.stream.pos();
        self.stream.seek(5);
        self.char5 = String::from_utf8_lossy(self.stream.read_bytes(1)?);
        self.stream.seek(_pos);
        return self.char5;
    }
    fn tuple5(&mut self) -> Box<Expr2__Tuple> {
        if let Some(x) = self.tuple5 {
            return x;
        }

        let _pos = self.stream.pos();
        self.stream.seek(5);
        self.tuple5 = Box::new(Expr2__Tuple::new(self.stream, self, _root)?);
        self.stream.seek(_pos);
        return self.tuple5;
    }
}
#[derive(Default)]
pub struct Expr2__Tuple {
    pub byte0: u8,
    pub byte1: u8,
    pub byte2: u8,
    pub avg: Option<i32>,
}

impl KaitaiStruct for Expr2__Tuple {
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
        self.byte0 = self.stream.read_u1()?;
        self.byte1 = self.stream.read_u1()?;
        self.byte2 = self.stream.read_u1()?;
    }
}

impl Expr2__Tuple {
    fn avg(&mut self) -> i32 {
        if let Some(x) = self.avg {
            return x;
        }

        self.avg = (self.byte1 + self.byte2) / 2;
        return self.avg;
    }
}
