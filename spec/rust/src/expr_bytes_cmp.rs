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

pub struct ExprBytesCmp {
    pub isLe: bool,
    pub ack: String,
    pub isGt2: bool,
    pub isGt: bool,
    pub ack2: String,
    pub isEq: bool,
    pub isLt2: bool,
    pub isGe: bool,
    pub hiVal: String,
    pub isNe: bool,
    pub isLt: bool,
    pub one: String,
    pub two: String,
}

impl KaitaiStruct for ExprBytesCmp {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            isLe: bool,
            ack: String,
            isGt2: bool,
            isGt: bool,
            ack2: String,
            isEq: bool,
            isLt2: bool,
            isGe: bool,
            hiVal: String,
            isNe: bool,
            isLt: bool,
            one: String,
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
        self.one = stream->readBytes(1);
        self.two = stream->readBytes(3);

        Ok(())
    }
    public function isLe() {
        if (self.isLe !== null)
            return self.isLe;
        self.isLe = $this->two() <= $this->ack2();
        return self.isLe;
    }
    public function ack() {
        if (self.ack !== null)
            return self.ack;
        self.ack = "\x41\x43\x4B";
        return self.ack;
    }
    public function isGt2() {
        if (self.isGt2 !== null)
            return self.isGt2;
        self.isGt2 = $this->hiVal() > $this->two();
        return self.isGt2;
    }
    public function isGt() {
        if (self.isGt !== null)
            return self.isGt;
        self.isGt = $this->two() > $this->ack2();
        return self.isGt;
    }
    public function ack2() {
        if (self.ack2 !== null)
            return self.ack2;
        self.ack2 = "\x41\x43\x4B\x32";
        return self.ack2;
    }
    public function isEq() {
        if (self.isEq !== null)
            return self.isEq;
        self.isEq = $this->two() == $this->ack();
        return self.isEq;
    }
    public function isLt2() {
        if (self.isLt2 !== null)
            return self.isLt2;
        self.isLt2 = $this->one() < $this->two();
        return self.isLt2;
    }
    public function isGe() {
        if (self.isGe !== null)
            return self.isGe;
        self.isGe = $this->two() >= $this->ack2();
        return self.isGe;
    }
    public function hiVal() {
        if (self.hiVal !== null)
            return self.hiVal;
        self.hiVal = "\x90\x43";
        return self.hiVal;
    }
    public function isNe() {
        if (self.isNe !== null)
            return self.isNe;
        self.isNe = $this->two() != $this->ack();
        return self.isNe;
    }
    public function isLt() {
        if (self.isLt !== null)
            return self.isLt;
        self.isLt = $this->two() < $this->ack2();
        return self.isLt;
    }
}
