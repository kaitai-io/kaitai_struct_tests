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

pub struct TypeIntUnaryOp {
    pub unaryS2: i32,
    pub unaryS8: i64,
    pub valueS2: i16,
    pub valueS8: i64,
}

impl KaitaiStruct for TypeIntUnaryOp {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            unaryS2: i32,
            unaryS8: 0,
            valueS2: 0,
            valueS8: 0,
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
        self.valueS2 = stream.read_s2le()?;
        self.valueS8 = stream.read_s8le()?;

        Ok(())
    }
    public function unaryS2() {
        if (self.unaryS2 !== null)
            return self.unaryS2;
        self.unaryS2 = -($this->valueS2());
        return self.unaryS2;
    }
    public function unaryS8() {
        if (self.unaryS8 !== null)
            return self.unaryS8;
        self.unaryS8 = -($this->valueS8());
        return self.unaryS8;
    }
}
