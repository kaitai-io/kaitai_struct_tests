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

pub struct Imports0 {
    pub hwOne: u8,
    pub two: u8,
    pub hw: ,
}

impl KaitaiStruct for Imports0 {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            hwOne: 0,
            two: 0,
            hw: ,
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
        self.two = stream.read_u1()?;
        self.hw = new hello_world(stream);

        Ok(())
    }
    public function hwOne() {
        if (self.hwOne !== null)
            return self.hwOne;
        self.hwOne = $this->hw()->one();
        return self.hwOne;
    }
}
