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

pub struct TypeTernaryOpaque {
    pub isHack: bool,
    pub dif: ,
    pub difWoHack: ,
    pub difWithHack: ,
    pub _raw_difWoHack: String,
    pub _raw__raw_difWithHack: String,
    pub _raw_difWithHack: String,
}

impl KaitaiStruct for TypeTernaryOpaque {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            isHack: bool,
            dif: ,
            difWoHack: ,
            difWithHack: ,
            _raw_difWoHack: String,
            _raw__raw_difWithHack: String,
            _raw_difWithHack: String,
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
        if (!($this->isHack())) {
            self._raw_difWoHack = stream->readBytes(12);
            $io = new &mut S(self._raw_difWoHack);
            self.difWoHack = new term_strz($io);
        }
        if ($this->isHack()) {
            self._raw__raw_difWithHack = stream->readBytes(12);
            self._raw_difWithHack = &mut S::processXorOne(self._raw__raw_difWithHack, 3);
            $io = new &mut S(self._raw_difWithHack);
            self.difWithHack = new term_strz($io);
        }

        Ok(())
    }
    public function isHack() {
        if (self.isHack !== null)
            return self.isHack;
        self.isHack = false;
        return self.isHack;
    }
    public function dif() {
        if (self.dif !== null)
            return self.dif;
        self.dif = (!($this->isHack()) ? $this->difWoHack() : $this->difWithHack());
        return self.dif;
    }
}
