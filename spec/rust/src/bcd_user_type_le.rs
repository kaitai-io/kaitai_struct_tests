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

pub struct BcdUserTypeLe {
pub struct LtrObj {
pub struct RtlObj {
pub struct LeadingZeroLtrObj {
    pub ltr: ,
    pub rtl: ,
    pub leadingZeroLtr: ,
    pub _raw_ltr: String,
    pub _raw_rtl: String,
    pub _raw_leadingZeroLtr: String,
    pub asInt: i32,
    pub digit2: i32,
    pub digit4: i32,
    pub digit3: i32,
    pub digit5: i32,
    pub digit8: i32,
    pub digit6: i32,
    pub asStr: String,
    pub digit1: i32,
    pub digit7: i32,
    pub b1: u8,
    pub b2: u8,
    pub b3: u8,
    pub b4: u8,
    pub asInt: i32,
    pub digit2: i32,
    pub digit4: i32,
    pub digit3: i32,
    pub digit5: i32,
    pub digit8: i32,
    pub digit6: i32,
    pub asStr: String,
    pub digit1: i32,
    pub digit7: i32,
    pub b1: u8,
    pub b2: u8,
    pub b3: u8,
    pub b4: u8,
    pub asInt: i32,
    pub digit2: i32,
    pub digit4: i32,
    pub digit3: i32,
    pub digit5: i32,
    pub digit8: i32,
    pub digit6: i32,
    pub asStr: String,
    pub digit1: i32,
    pub digit7: i32,
    pub b1: u8,
    pub b2: u8,
    pub b3: u8,
    pub b4: u8,
}

impl KaitaiStruct for BcdUserTypeLe {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for LtrObj {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
                }

                impl KaitaiStruct for RtlObj {
                    fn new<S: KaitaiStream>(stream: &mut S,
                                            _parent: &Option<Box<KaitaiStruct>>,
                                            _root: &Option<Box<KaitaiStruct>>)
                                            -> Result<Self>
                        where Self: Sized {
                        let mut s = Self {
                        }

                        impl KaitaiStruct for LeadingZeroLtrObj {
                            fn new<S: KaitaiStream>(stream: &mut S,
                                                    _parent: &Option<Box<KaitaiStruct>>,
                                                    _root: &Option<Box<KaitaiStruct>>)
                                                    -> Result<Self>
                                where Self: Sized {
                                let mut s = Self {
            ltr: ,
            rtl: ,
            leadingZeroLtr: ,
            _raw_ltr: String,
            _raw_rtl: String,
            _raw_leadingZeroLtr: String,
            asInt: i32,
            digit2: i32,
            digit4: i32,
            digit3: i32,
            digit5: i32,
            digit8: i32,
            digit6: i32,
            asStr: String,
            digit1: i32,
            digit7: i32,
            b1: 0,
            b2: 0,
            b3: 0,
            b4: 0,
            asInt: i32,
            digit2: i32,
            digit4: i32,
            digit3: i32,
            digit5: i32,
            digit8: i32,
            digit6: i32,
            asStr: String,
            digit1: i32,
            digit7: i32,
            b1: 0,
            b2: 0,
            b3: 0,
            b4: 0,
            asInt: i32,
            digit2: i32,
            digit4: i32,
            digit3: i32,
            digit5: i32,
            digit8: i32,
            digit6: i32,
            asStr: String,
            digit1: i32,
            digit7: i32,
            b1: 0,
            b2: 0,
            b3: 0,
            b4: 0,
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
        self._raw_ltr = stream->readBytes(4);
        $io = new &mut S(self._raw_ltr);
        self.ltr = new bcd_user_type_le::ltr_obj($io, $this, _root);
        self._raw_rtl = stream->readBytes(4);
        $io = new &mut S(self._raw_rtl);
        self.rtl = new bcd_user_type_le::rtl_obj($io, $this, _root);
        self._raw_leadingZeroLtr = stream->readBytes(4);
        $io = new &mut S(self._raw_leadingZeroLtr);
        self.leadingZeroLtr = new bcd_user_type_le::leading_zero_ltr_obj($io, $this, _root);

        Ok(())
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
        self.b1 = stream.read_u1()?;
        self.b2 = stream.read_u1()?;
        self.b3 = stream.read_u1()?;
        self.b4 = stream.read_u1()?;

        Ok(())
    }
    public function asInt() {
        if (self.asInt !== null)
            return self.asInt;
        self.asInt = (((((((($this->digit8() * 1) + ($this->digit7() * 10)) + ($this->digit6() * 100)) + ($this->digit5() * 1000)) + ($this->digit4() * 10000)) + ($this->digit3() * 100000)) + ($this->digit2() * 1000000)) + ($this->digit1() * 10000000));
        return self.asInt;
    }
    public function digit2() {
        if (self.digit2 !== null)
            return self.digit2;
        self.digit2 = ($this->b4() & 15);
        return self.digit2;
    }
    public function digit4() {
        if (self.digit4 !== null)
            return self.digit4;
        self.digit4 = ($this->b3() & 15);
        return self.digit4;
    }
    public function digit3() {
        if (self.digit3 !== null)
            return self.digit3;
        self.digit3 = (($this->b3() & 240) >> 4);
        return self.digit3;
    }
    public function digit5() {
        if (self.digit5 !== null)
            return self.digit5;
        self.digit5 = (($this->b2() & 240) >> 4);
        return self.digit5;
    }
    public function digit8() {
        if (self.digit8 !== null)
            return self.digit8;
        self.digit8 = ($this->b1() & 15);
        return self.digit8;
    }
    public function digit6() {
        if (self.digit6 !== null)
            return self.digit6;
        self.digit6 = ($this->b2() & 15);
        return self.digit6;
    }
    public function asStr() {
        if (self.asStr !== null)
            return self.asStr;
        self.asStr = strval($this->digit1()) . strval($this->digit2()) . strval($this->digit3()) . strval($this->digit4()) . strval($this->digit5()) . strval($this->digit6()) . strval($this->digit7()) . strval($this->digit8());
        return self.asStr;
    }
    public function digit1() {
        if (self.digit1 !== null)
            return self.digit1;
        self.digit1 = (($this->b4() & 240) >> 4);
        return self.digit1;
    }
    public function digit7() {
        if (self.digit7 !== null)
            return self.digit7;
        self.digit7 = (($this->b1() & 240) >> 4);
        return self.digit7;
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
        self.b1 = stream.read_u1()?;
        self.b2 = stream.read_u1()?;
        self.b3 = stream.read_u1()?;
        self.b4 = stream.read_u1()?;

        Ok(())
    }
    public function asInt() {
        if (self.asInt !== null)
            return self.asInt;
        self.asInt = (((((((($this->digit1() * 1) + ($this->digit2() * 10)) + ($this->digit3() * 100)) + ($this->digit4() * 1000)) + ($this->digit5() * 10000)) + ($this->digit6() * 100000)) + ($this->digit7() * 1000000)) + ($this->digit8() * 10000000));
        return self.asInt;
    }
    public function digit2() {
        if (self.digit2 !== null)
            return self.digit2;
        self.digit2 = ($this->b4() & 15);
        return self.digit2;
    }
    public function digit4() {
        if (self.digit4 !== null)
            return self.digit4;
        self.digit4 = ($this->b3() & 15);
        return self.digit4;
    }
    public function digit3() {
        if (self.digit3 !== null)
            return self.digit3;
        self.digit3 = (($this->b3() & 240) >> 4);
        return self.digit3;
    }
    public function digit5() {
        if (self.digit5 !== null)
            return self.digit5;
        self.digit5 = (($this->b2() & 240) >> 4);
        return self.digit5;
    }
    public function digit8() {
        if (self.digit8 !== null)
            return self.digit8;
        self.digit8 = ($this->b1() & 15);
        return self.digit8;
    }
    public function digit6() {
        if (self.digit6 !== null)
            return self.digit6;
        self.digit6 = ($this->b2() & 15);
        return self.digit6;
    }
    public function asStr() {
        if (self.asStr !== null)
            return self.asStr;
        self.asStr = strval($this->digit8()) . strval($this->digit7()) . strval($this->digit6()) . strval($this->digit5()) . strval($this->digit4()) . strval($this->digit3()) . strval($this->digit2()) . strval($this->digit1());
        return self.asStr;
    }
    public function digit1() {
        if (self.digit1 !== null)
            return self.digit1;
        self.digit1 = (($this->b4() & 240) >> 4);
        return self.digit1;
    }
    public function digit7() {
        if (self.digit7 !== null)
            return self.digit7;
        self.digit7 = (($this->b1() & 240) >> 4);
        return self.digit7;
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
        self.b1 = stream.read_u1()?;
        self.b2 = stream.read_u1()?;
        self.b3 = stream.read_u1()?;
        self.b4 = stream.read_u1()?;

        Ok(())
    }
    public function asInt() {
        if (self.asInt !== null)
            return self.asInt;
        self.asInt = (((((((($this->digit8() * 1) + ($this->digit7() * 10)) + ($this->digit6() * 100)) + ($this->digit5() * 1000)) + ($this->digit4() * 10000)) + ($this->digit3() * 100000)) + ($this->digit2() * 1000000)) + ($this->digit1() * 10000000));
        return self.asInt;
    }
    public function digit2() {
        if (self.digit2 !== null)
            return self.digit2;
        self.digit2 = ($this->b4() & 15);
        return self.digit2;
    }
    public function digit4() {
        if (self.digit4 !== null)
            return self.digit4;
        self.digit4 = ($this->b3() & 15);
        return self.digit4;
    }
    public function digit3() {
        if (self.digit3 !== null)
            return self.digit3;
        self.digit3 = (($this->b3() & 240) >> 4);
        return self.digit3;
    }
    public function digit5() {
        if (self.digit5 !== null)
            return self.digit5;
        self.digit5 = (($this->b2() & 240) >> 4);
        return self.digit5;
    }
    public function digit8() {
        if (self.digit8 !== null)
            return self.digit8;
        self.digit8 = ($this->b1() & 15);
        return self.digit8;
    }
    public function digit6() {
        if (self.digit6 !== null)
            return self.digit6;
        self.digit6 = ($this->b2() & 15);
        return self.digit6;
    }
    public function asStr() {
        if (self.asStr !== null)
            return self.asStr;
        self.asStr = strval($this->digit1()) . strval($this->digit2()) . strval($this->digit3()) . strval($this->digit4()) . strval($this->digit5()) . strval($this->digit6()) . strval($this->digit7()) . strval($this->digit8());
        return self.asStr;
    }
    public function digit1() {
        if (self.digit1 !== null)
            return self.digit1;
        self.digit1 = (($this->b4() & 240) >> 4);
        return self.digit1;
    }
    public function digit7() {
        if (self.digit7 !== null)
            return self.digit7;
        self.digit7 = (($this->b1() & 240) >> 4);
        return self.digit7;
    }
}
