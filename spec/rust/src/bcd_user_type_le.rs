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
pub struct BcdUserTypeLe {
    pub ltr: Box<BcdUserTypeLe__LtrObj>,
    pub rtl: Box<BcdUserTypeLe__RtlObj>,
    pub leadingZeroLtr: Box<BcdUserTypeLe__LeadingZeroLtrObj>,
    pub _raw_ltr: Vec<u8>,
    pub _raw_rtl: Vec<u8>,
    pub _raw_leadingZeroLtr: Vec<u8>,
}

impl KaitaiStruct for BcdUserTypeLe {
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
        self._raw_ltr = self.stream.read_bytes(4)?;
        let mut io = Cursor::new(self._raw_ltr);
        self.ltr = Box::new(BcdUserTypeLe__LtrObj::new(self.stream, self, _root)?);
        self._raw_rtl = self.stream.read_bytes(4)?;
        let mut io = Cursor::new(self._raw_rtl);
        self.rtl = Box::new(BcdUserTypeLe__RtlObj::new(self.stream, self, _root)?);
        self._raw_leadingZeroLtr = self.stream.read_bytes(4)?;
        let mut io = Cursor::new(self._raw_leadingZeroLtr);
        self.leadingZeroLtr = Box::new(BcdUserTypeLe__LeadingZeroLtrObj::new(self.stream, self, _root)?);
    }
}

impl BcdUserTypeLe {
}
#[derive(Default)]
pub struct BcdUserTypeLe__LtrObj {
    pub b1: u8,
    pub b2: u8,
    pub b3: u8,
    pub b4: u8,
    pub asInt: Option<i32>,
    pub digit2: Option<i32>,
    pub digit4: Option<i32>,
    pub digit3: Option<i32>,
    pub digit5: Option<i32>,
    pub digit8: Option<i32>,
    pub digit6: Option<i32>,
    pub asStr: Option<String>,
    pub digit1: Option<i32>,
    pub digit7: Option<i32>,
}

impl KaitaiStruct for BcdUserTypeLe__LtrObj {
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
        self.b1 = self.stream.read_u1()?;
        self.b2 = self.stream.read_u1()?;
        self.b3 = self.stream.read_u1()?;
        self.b4 = self.stream.read_u1()?;
    }
}

impl BcdUserTypeLe__LtrObj {
    fn asInt(&mut self) -> i32 {
        if let Some(x) = self.asInt {
            return x;
        }

        self.asInt = ((((((((self.digit8 * 1) + (self.digit7 * 10)) + (self.digit6 * 100)) + (self.digit5 * 1000)) + (self.digit4 * 10000)) + (self.digit3 * 100000)) + (self.digit2 * 1000000)) + (self.digit1 * 10000000));
        return self.asInt;
    }
    fn digit2(&mut self) -> i32 {
        if let Some(x) = self.digit2 {
            return x;
        }

        self.digit2 = (self.b4 & 15);
        return self.digit2;
    }
    fn digit4(&mut self) -> i32 {
        if let Some(x) = self.digit4 {
            return x;
        }

        self.digit4 = (self.b3 & 15);
        return self.digit4;
    }
    fn digit3(&mut self) -> i32 {
        if let Some(x) = self.digit3 {
            return x;
        }

        self.digit3 = ((self.b3 & 240) >> 4);
        return self.digit3;
    }
    fn digit5(&mut self) -> i32 {
        if let Some(x) = self.digit5 {
            return x;
        }

        self.digit5 = ((self.b2 & 240) >> 4);
        return self.digit5;
    }
    fn digit8(&mut self) -> i32 {
        if let Some(x) = self.digit8 {
            return x;
        }

        self.digit8 = (self.b1 & 15);
        return self.digit8;
    }
    fn digit6(&mut self) -> i32 {
        if let Some(x) = self.digit6 {
            return x;
        }

        self.digit6 = (self.b2 & 15);
        return self.digit6;
    }
    fn asStr(&mut self) -> String {
        if let Some(x) = self.asStr {
            return x;
        }

        self.asStr = format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", self.digit1.to_string(), self.digit2.to_string()), self.digit3.to_string()), self.digit4.to_string()), self.digit5.to_string()), self.digit6.to_string()), self.digit7.to_string()), self.digit8.to_string());
        return self.asStr;
    }
    fn digit1(&mut self) -> i32 {
        if let Some(x) = self.digit1 {
            return x;
        }

        self.digit1 = ((self.b4 & 240) >> 4);
        return self.digit1;
    }
    fn digit7(&mut self) -> i32 {
        if let Some(x) = self.digit7 {
            return x;
        }

        self.digit7 = ((self.b1 & 240) >> 4);
        return self.digit7;
    }
}
#[derive(Default)]
pub struct BcdUserTypeLe__RtlObj {
    pub b1: u8,
    pub b2: u8,
    pub b3: u8,
    pub b4: u8,
    pub asInt: Option<i32>,
    pub digit2: Option<i32>,
    pub digit4: Option<i32>,
    pub digit3: Option<i32>,
    pub digit5: Option<i32>,
    pub digit8: Option<i32>,
    pub digit6: Option<i32>,
    pub asStr: Option<String>,
    pub digit1: Option<i32>,
    pub digit7: Option<i32>,
}

impl KaitaiStruct for BcdUserTypeLe__RtlObj {
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
        self.b1 = self.stream.read_u1()?;
        self.b2 = self.stream.read_u1()?;
        self.b3 = self.stream.read_u1()?;
        self.b4 = self.stream.read_u1()?;
    }
}

impl BcdUserTypeLe__RtlObj {
    fn asInt(&mut self) -> i32 {
        if let Some(x) = self.asInt {
            return x;
        }

        self.asInt = ((((((((self.digit1 * 1) + (self.digit2 * 10)) + (self.digit3 * 100)) + (self.digit4 * 1000)) + (self.digit5 * 10000)) + (self.digit6 * 100000)) + (self.digit7 * 1000000)) + (self.digit8 * 10000000));
        return self.asInt;
    }
    fn digit2(&mut self) -> i32 {
        if let Some(x) = self.digit2 {
            return x;
        }

        self.digit2 = (self.b4 & 15);
        return self.digit2;
    }
    fn digit4(&mut self) -> i32 {
        if let Some(x) = self.digit4 {
            return x;
        }

        self.digit4 = (self.b3 & 15);
        return self.digit4;
    }
    fn digit3(&mut self) -> i32 {
        if let Some(x) = self.digit3 {
            return x;
        }

        self.digit3 = ((self.b3 & 240) >> 4);
        return self.digit3;
    }
    fn digit5(&mut self) -> i32 {
        if let Some(x) = self.digit5 {
            return x;
        }

        self.digit5 = ((self.b2 & 240) >> 4);
        return self.digit5;
    }
    fn digit8(&mut self) -> i32 {
        if let Some(x) = self.digit8 {
            return x;
        }

        self.digit8 = (self.b1 & 15);
        return self.digit8;
    }
    fn digit6(&mut self) -> i32 {
        if let Some(x) = self.digit6 {
            return x;
        }

        self.digit6 = (self.b2 & 15);
        return self.digit6;
    }
    fn asStr(&mut self) -> String {
        if let Some(x) = self.asStr {
            return x;
        }

        self.asStr = format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", self.digit8.to_string(), self.digit7.to_string()), self.digit6.to_string()), self.digit5.to_string()), self.digit4.to_string()), self.digit3.to_string()), self.digit2.to_string()), self.digit1.to_string());
        return self.asStr;
    }
    fn digit1(&mut self) -> i32 {
        if let Some(x) = self.digit1 {
            return x;
        }

        self.digit1 = ((self.b4 & 240) >> 4);
        return self.digit1;
    }
    fn digit7(&mut self) -> i32 {
        if let Some(x) = self.digit7 {
            return x;
        }

        self.digit7 = ((self.b1 & 240) >> 4);
        return self.digit7;
    }
}
#[derive(Default)]
pub struct BcdUserTypeLe__LeadingZeroLtrObj {
    pub b1: u8,
    pub b2: u8,
    pub b3: u8,
    pub b4: u8,
    pub asInt: Option<i32>,
    pub digit2: Option<i32>,
    pub digit4: Option<i32>,
    pub digit3: Option<i32>,
    pub digit5: Option<i32>,
    pub digit8: Option<i32>,
    pub digit6: Option<i32>,
    pub asStr: Option<String>,
    pub digit1: Option<i32>,
    pub digit7: Option<i32>,
}

impl KaitaiStruct for BcdUserTypeLe__LeadingZeroLtrObj {
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
        self.b1 = self.stream.read_u1()?;
        self.b2 = self.stream.read_u1()?;
        self.b3 = self.stream.read_u1()?;
        self.b4 = self.stream.read_u1()?;
    }
}

impl BcdUserTypeLe__LeadingZeroLtrObj {
    fn asInt(&mut self) -> i32 {
        if let Some(x) = self.asInt {
            return x;
        }

        self.asInt = ((((((((self.digit8 * 1) + (self.digit7 * 10)) + (self.digit6 * 100)) + (self.digit5 * 1000)) + (self.digit4 * 10000)) + (self.digit3 * 100000)) + (self.digit2 * 1000000)) + (self.digit1 * 10000000));
        return self.asInt;
    }
    fn digit2(&mut self) -> i32 {
        if let Some(x) = self.digit2 {
            return x;
        }

        self.digit2 = (self.b4 & 15);
        return self.digit2;
    }
    fn digit4(&mut self) -> i32 {
        if let Some(x) = self.digit4 {
            return x;
        }

        self.digit4 = (self.b3 & 15);
        return self.digit4;
    }
    fn digit3(&mut self) -> i32 {
        if let Some(x) = self.digit3 {
            return x;
        }

        self.digit3 = ((self.b3 & 240) >> 4);
        return self.digit3;
    }
    fn digit5(&mut self) -> i32 {
        if let Some(x) = self.digit5 {
            return x;
        }

        self.digit5 = ((self.b2 & 240) >> 4);
        return self.digit5;
    }
    fn digit8(&mut self) -> i32 {
        if let Some(x) = self.digit8 {
            return x;
        }

        self.digit8 = (self.b1 & 15);
        return self.digit8;
    }
    fn digit6(&mut self) -> i32 {
        if let Some(x) = self.digit6 {
            return x;
        }

        self.digit6 = (self.b2 & 15);
        return self.digit6;
    }
    fn asStr(&mut self) -> String {
        if let Some(x) = self.asStr {
            return x;
        }

        self.asStr = format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", self.digit1.to_string(), self.digit2.to_string()), self.digit3.to_string()), self.digit4.to_string()), self.digit5.to_string()), self.digit6.to_string()), self.digit7.to_string()), self.digit8.to_string());
        return self.asStr;
    }
    fn digit1(&mut self) -> i32 {
        if let Some(x) = self.digit1 {
            return x;
        }

        self.digit1 = ((self.b4 & 240) >> 4);
        return self.digit1;
    }
    fn digit7(&mut self) -> i32 {
        if let Some(x) = self.digit7 {
            return x;
        }

        self.digit7 = ((self.b1 & 240) >> 4);
        return self.digit7;
    }
}
