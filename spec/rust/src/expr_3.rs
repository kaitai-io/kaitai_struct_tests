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

pub struct Expr3 {
    pub three: String,
    pub isStrGe: bool,
    pub isStrNe: bool,
    pub isStrGt: bool,
    pub isStrLe: bool,
    pub isStrLt2: bool,
    pub testNot: bool,
    pub isStrLt: bool,
    pub four: String,
    pub isStrEq: bool,
    pub one: u8,
    pub two: String,
}

impl KaitaiStruct for Expr3 {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            three: String,
            isStrGe: bool,
            isStrNe: bool,
            isStrGt: bool,
            isStrLe: bool,
            isStrLt2: bool,
            testNot: bool,
            isStrLt: bool,
            four: String,
            isStrEq: bool,
            one: 0,
            two: String,
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
        self.one = stream.read_u1()?;
        self.two = &mut S::bytesToStr(stream->readBytes(3), "ASCII");

        Ok(())
    }
    public function three() {
        if (self.three !== null)
            return self.three;
        self.three = "@" . $this->two();
        return self.three;
    }
    public function isStrGe() {
        if (self.isStrGe !== null)
            return self.isStrGe;
        self.isStrGe = $this->two() >= "ACK2";
        return self.isStrGe;
    }
    public function isStrNe() {
        if (self.isStrNe !== null)
            return self.isStrNe;
        self.isStrNe = $this->two() != "ACK";
        return self.isStrNe;
    }
    public function isStrGt() {
        if (self.isStrGt !== null)
            return self.isStrGt;
        self.isStrGt = $this->two() > "ACK2";
        return self.isStrGt;
    }
    public function isStrLe() {
        if (self.isStrLe !== null)
            return self.isStrLe;
        self.isStrLe = $this->two() <= "ACK2";
        return self.isStrLe;
    }
    public function isStrLt2() {
        if (self.isStrLt2 !== null)
            return self.isStrLt2;
        self.isStrLt2 = $this->three() < $this->two();
        return self.isStrLt2;
    }
    public function testNot() {
        if (self.testNot !== null)
            return self.testNot;
        self.testNot = !(false);
        return self.testNot;
    }
    public function isStrLt() {
        if (self.isStrLt !== null)
            return self.isStrLt;
        self.isStrLt = $this->two() < "ACK2";
        return self.isStrLt;
    }
    public function four() {
        if (self.four !== null)
            return self.four;
        self.four = "_" . $this->two() . "_";
        return self.four;
    }
    public function isStrEq() {
        if (self.isStrEq !== null)
            return self.isStrEq;
        self.isStrEq = $this->two() == "ACK";
        return self.isStrEq;
    }
}
