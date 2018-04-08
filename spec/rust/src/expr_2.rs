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

pub struct Expr2 {
pub struct ModStr {
pub struct Tuple {
    pub str1LenMod: i32,
    pub str1Len: i32,
    pub str1Tuple5: ,
    pub str2Tuple5: ,
    pub str1Avg: i32,
    pub str1Byte1: u8,
    pub str1Char5: String,
    pub str1: ,
    pub str2: ,
    pub lenMod: i32,
    pub char5: String,
    pub tuple5: ,
    pub lenOrig: u16,
    pub str: String,
    pub rest: ,
    pub _raw_rest: String,
    pub avg: i32,
    pub byte0: u8,
    pub byte1: u8,
    pub byte2: u8,
}

impl KaitaiStruct for Expr2 {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for ModStr {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
                }

                impl KaitaiStruct for Tuple {
                    fn new<S: KaitaiStream>(stream: &mut S,
                                            _parent: &Option<Box<KaitaiStruct>>,
                                            _root: &Option<Box<KaitaiStruct>>)
                                            -> Result<Self>
                        where Self: Sized {
                        let mut s = Self {
            str1LenMod: i32,
            str1Len: i32,
            str1Tuple5: ,
            str2Tuple5: ,
            str1Avg: i32,
            str1Byte1: 0,
            str1Char5: String,
            str1: ,
            str2: ,
            lenMod: i32,
            char5: String,
            tuple5: ,
            lenOrig: 0,
            str: String,
            rest: ,
            _raw_rest: String,
            avg: i32,
            byte0: 0,
            byte1: 0,
            byte2: 0,
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
        self.str1 = new expr_2::mod_str(stream, $this, _root);
        self.str2 = new expr_2::mod_str(stream, $this, _root);

        Ok(())
    }
    public function str1LenMod() {
        if (self.str1LenMod !== null)
            return self.str1LenMod;
        self.str1LenMod = $this->str1()->lenMod();
        return self.str1LenMod;
    }
    public function str1Len() {
        if (self.str1Len !== null)
            return self.str1Len;
        self.str1Len = strlen($this->str1()->str());
        return self.str1Len;
    }
    public function str1Tuple5() {
        if (self.str1Tuple5 !== null)
            return self.str1Tuple5;
        self.str1Tuple5 = $this->str1()->tuple5();
        return self.str1Tuple5;
    }
    public function str2Tuple5() {
        if (self.str2Tuple5 !== null)
            return self.str2Tuple5;
        self.str2Tuple5 = $this->str2()->tuple5();
        return self.str2Tuple5;
    }
    public function str1Avg() {
        if (self.str1Avg !== null)
            return self.str1Avg;
        self.str1Avg = $this->str1()->rest()->avg();
        return self.str1Avg;
    }
    public function str1Byte1() {
        if (self.str1Byte1 !== null)
            return self.str1Byte1;
        self.str1Byte1 = $this->str1()->rest()->byte1();
        return self.str1Byte1;
    }
    public function str1Char5() {
        if (self.str1Char5 !== null)
            return self.str1Char5;
        self.str1Char5 = $this->str1()->char5();
        return self.str1Char5;
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
        self.lenOrig = stream.read_u2le()?;
        self.str = &mut S::bytesToStr(stream->readBytes($this->lenMod()), "UTF-8");
        self._raw_rest = stream->readBytes(3);
        $io = new &mut S(self._raw_rest);
        self.rest = new expr_2::tuple($io, $this, _root);

        Ok(())
    }
    public function lenMod() {
        if (self.lenMod !== null)
            return self.lenMod;
        self.lenMod = ($this->lenOrig() - 3);
        return self.lenMod;
    }
    public function char5() {
        if (self.char5 !== null)
            return self.char5;
        $_pos = stream->pos();
        stream->seek(5);
        self.char5 = &mut S::bytesToStr(stream->readBytes(1), "ASCII");
        stream->seek($_pos);
        return self.char5;
    }
    public function tuple5() {
        if (self.tuple5 !== null)
            return self.tuple5;
        $_pos = stream->pos();
        stream->seek(5);
        self.tuple5 = new expr_2::tuple(stream, $this, _root);
        stream->seek($_pos);
        return self.tuple5;
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
        self.byte0 = stream.read_u1()?;
        self.byte1 = stream.read_u1()?;
        self.byte2 = stream.read_u1()?;

        Ok(())
    }
    public function avg() {
        if (self.avg !== null)
            return self.avg;
        self.avg = intval(($this->byte1() + $this->byte2()) / 2);
        return self.avg;
    }
}
