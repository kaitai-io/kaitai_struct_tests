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
pub struct ExprBytesCmp {
    pub one: Vec<u8>,
    pub two: Vec<u8>,
    pub isLe: Option<bool>,
    pub ack: Option<Vec<u8>>,
    pub isGt2: Option<bool>,
    pub isGt: Option<bool>,
    pub ack2: Option<Vec<u8>>,
    pub isEq: Option<bool>,
    pub isLt2: Option<bool>,
    pub isGe: Option<bool>,
    pub hiVal: Option<Vec<u8>>,
    pub isNe: Option<bool>,
    pub isLt: Option<bool>,
}

impl KaitaiStruct for ExprBytesCmp {
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
        self.one = self.stream.read_bytes(1)?;
        self.two = self.stream.read_bytes(3)?;
    }
}

impl ExprBytesCmp {
    fn isLe(&mut self) -> bool {
        if let Some(x) = self.isLe {
            return x;
        }

        self.isLe = self.two <= self.ack2;
        return self.isLe;
    }
    fn ack(&mut self) -> Vec<u8> {
        if let Some(x) = self.ack {
            return x;
        }

        self.ack = vec!([0x41, 0x43, 0x4b]);
        return self.ack;
    }
    fn isGt2(&mut self) -> bool {
        if let Some(x) = self.isGt2 {
            return x;
        }

        self.isGt2 = self.hi_val > self.two;
        return self.isGt2;
    }
    fn isGt(&mut self) -> bool {
        if let Some(x) = self.isGt {
            return x;
        }

        self.isGt = self.two > self.ack2;
        return self.isGt;
    }
    fn ack2(&mut self) -> Vec<u8> {
        if let Some(x) = self.ack2 {
            return x;
        }

        self.ack2 = vec!([0x41, 0x43, 0x4b, 0x32]);
        return self.ack2;
    }
    fn isEq(&mut self) -> bool {
        if let Some(x) = self.isEq {
            return x;
        }

        self.isEq = self.two == self.ack;
        return self.isEq;
    }
    fn isLt2(&mut self) -> bool {
        if let Some(x) = self.isLt2 {
            return x;
        }

        self.isLt2 = self.one < self.two;
        return self.isLt2;
    }
    fn isGe(&mut self) -> bool {
        if let Some(x) = self.isGe {
            return x;
        }

        self.isGe = self.two >= self.ack2;
        return self.isGe;
    }
    fn hiVal(&mut self) -> Vec<u8> {
        if let Some(x) = self.hiVal {
            return x;
        }

        self.hiVal = vec!([0x90, 0x43]);
        return self.hiVal;
    }
    fn isNe(&mut self) -> bool {
        if let Some(x) = self.isNe {
            return x;
        }

        self.isNe = self.two != self.ack;
        return self.isNe;
    }
    fn isLt(&mut self) -> bool {
        if let Some(x) = self.isLt {
            return x;
        }

        self.isLt = self.two < self.ack2;
        return self.isLt;
    }
}
