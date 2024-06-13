// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct Expr3 {
    pub one: u8,
    pub two: String,
    pub three: Option<String>,
    pub isStrGe: Option<bool>,
    pub isStrNe: Option<bool>,
    pub isStrGt: Option<bool>,
    pub isStrLe: Option<bool>,
    pub isStrLt2: Option<bool>,
    pub testNot: Option<bool>,
    pub isStrLt: Option<bool>,
    pub four: Option<String>,
    pub isStrEq: Option<bool>,
}

impl KaitaiStruct for Expr3 {
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
        self.one = self.stream.read_u1()?;
        self.two = String::from_utf8_lossy(self.stream.read_bytes(3)?);
    }
}

impl Expr3 {
    fn three(&mut self) -> String {
        if let Some(x) = self.three {
            return x;
        }

        self.three = format!("{}{}", "@", self.two);
        return self.three;
    }
    fn isStrGe(&mut self) -> bool {
        if let Some(x) = self.isStrGe {
            return x;
        }

        self.isStrGe = self.two >= "ACK2";
        return self.isStrGe;
    }
    fn isStrNe(&mut self) -> bool {
        if let Some(x) = self.isStrNe {
            return x;
        }

        self.isStrNe = self.two != "ACK";
        return self.isStrNe;
    }
    fn isStrGt(&mut self) -> bool {
        if let Some(x) = self.isStrGt {
            return x;
        }

        self.isStrGt = self.two > "ACK2";
        return self.isStrGt;
    }
    fn isStrLe(&mut self) -> bool {
        if let Some(x) = self.isStrLe {
            return x;
        }

        self.isStrLe = self.two <= "ACK2";
        return self.isStrLe;
    }
    fn isStrLt2(&mut self) -> bool {
        if let Some(x) = self.isStrLt2 {
            return x;
        }

        self.isStrLt2 = self.three < self.two;
        return self.isStrLt2;
    }
    fn testNot(&mut self) -> bool {
        if let Some(x) = self.testNot {
            return x;
        }

        self.testNot = !(false);
        return self.testNot;
    }
    fn isStrLt(&mut self) -> bool {
        if let Some(x) = self.isStrLt {
            return x;
        }

        self.isStrLt = self.two < "ACK2";
        return self.isStrLt;
    }
    fn four(&mut self) -> String {
        if let Some(x) = self.four {
            return x;
        }

        self.four = format!("{}{}", format!("{}{}", "_", self.two), "_");
        return self.four;
    }
    fn isStrEq(&mut self) -> bool {
        if let Some(x) = self.isStrEq {
            return x;
        }

        self.isStrEq = self.two == "ACK";
        return self.isStrEq;
    }
}
