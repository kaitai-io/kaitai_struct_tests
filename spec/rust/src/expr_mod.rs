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

pub struct ExprMod {
    pub modPosConst: i32,
    pub modNegConst: i32,
    pub modPosSeq: i32,
    pub modNegSeq: i32,
    pub intU: u32,
    pub intS: i32,
}

impl KaitaiStruct for ExprMod {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            modPosConst: i32,
            modNegConst: i32,
            modPosSeq: i32,
            modNegSeq: i32,
            intU: 0,
            intS: 0,
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
        self.intU = stream.read_u4le()?;
        self.intS = stream.read_s4le()?;

        Ok(())
    }
    public function modPosConst() {
        if (self.modPosConst !== null)
            return self.modPosConst;
        self.modPosConst = &mut S::mod(9837, 13);
        return self.modPosConst;
    }
    public function modNegConst() {
        if (self.modNegConst !== null)
            return self.modNegConst;
        self.modNegConst = &mut S::mod(-9837, 13);
        return self.modNegConst;
    }
    public function modPosSeq() {
        if (self.modPosSeq !== null)
            return self.modPosSeq;
        self.modPosSeq = &mut S::mod($this->intU(), 13);
        return self.modPosSeq;
    }
    public function modNegSeq() {
        if (self.modNegSeq !== null)
            return self.modNegSeq;
        self.modNegSeq = &mut S::mod($this->intS(), 13);
        return self.modNegSeq;
    }
}
