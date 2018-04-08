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

pub struct SwitchIntegers2 {
    pub lenModStr: String,
    pub code: u8,
    pub len: ,
    pub ham: String,
    pub padding: u8,
}

impl KaitaiStruct for SwitchIntegers2 {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            lenModStr: String,
            code: 0,
            len: ,
            ham: String,
            padding: 0,
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
        self.code = stream.read_u1()?;
        switch ($this->code()) {
            case 1:
                self.len = stream.read_u1()?;
                break;
            case 2:
                self.len = stream.read_u2le()?;
                break;
            case 4:
                self.len = stream.read_u4le()?;
                break;
            case 8:
                self.len = stream.read_u8le()?;
                break;
        }
        self.ham = stream->readBytes($this->len());
        if ($this->len() > 3) {
            self.padding = stream.read_u1()?;
        }

        Ok(())
    }
    public function lenModStr() {
        if (self.lenModStr !== null)
            return self.lenModStr;
        self.lenModStr = strval((($this->len() * 2) - 1));
        return self.lenModStr;
    }
}
