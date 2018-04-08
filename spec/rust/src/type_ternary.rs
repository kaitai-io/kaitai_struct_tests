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

pub struct TypeTernary {
pub struct Dummy {
    pub isHack: bool,
    pub dif: ,
    pub difValue: u8,
    pub difWoHack: ,
    pub difWithHack: ,
    pub _raw_difWoHack: String,
    pub _raw__raw_difWithHack: String,
    pub _raw_difWithHack: String,
    pub value: u8,
}

impl KaitaiStruct for TypeTernary {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for Dummy {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
            isHack: bool,
            dif: ,
            difValue: 0,
            difWoHack: ,
            difWithHack: ,
            _raw_difWoHack: String,
            _raw__raw_difWithHack: String,
            _raw_difWithHack: String,
            value: 0,
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
            self._raw_difWoHack = stream->readBytes(1);
            $io = new &mut S(self._raw_difWoHack);
            self.difWoHack = new type_ternary::dummy($io, $this, _root);
        }
        self._raw__raw_difWithHack = stream->readBytes(1);
        self._raw_difWithHack = &mut S::processXorOne(self._raw__raw_difWithHack, 3);
        $io = new &mut S(self._raw_difWithHack);
        self.difWithHack = new type_ternary::dummy($io, $this, _root);

        Ok(())
    }
    public function isHack() {
        if (self.isHack !== null)
            return self.isHack;
        self.isHack = true;
        return self.isHack;
    }
    public function dif() {
        if (self.dif !== null)
            return self.dif;
        self.dif = (!($this->isHack()) ? $this->difWoHack() : $this->difWithHack());
        return self.dif;
    }
    public function difValue() {
        if (self.difValue !== null)
            return self.difValue;
        self.difValue = $this->dif()->value();
        return self.difValue;
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
        self.value = stream.read_u1()?;

        Ok(())
    }
}
