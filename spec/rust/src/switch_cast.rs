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

pub struct SwitchCast {
pub struct Opcode {
pub struct Intval {
pub struct Strval {
    pub firstObj: ,
    pub secondVal: u8,
    pub errCast: ,
    pub opcodes: Vec<>*,
    pub code: u8,
    pub body: ,
    pub value: u8,
    pub value: String,
}

impl KaitaiStruct for SwitchCast {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for Opcode {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
                }

                impl KaitaiStruct for Intval {
                    fn new<S: KaitaiStream>(stream: &mut S,
                                            _parent: &Option<Box<KaitaiStruct>>,
                                            _root: &Option<Box<KaitaiStruct>>)
                                            -> Result<Self>
                        where Self: Sized {
                        let mut s = Self {
                        }

                        impl KaitaiStruct for Strval {
                            fn new<S: KaitaiStream>(stream: &mut S,
                                                    _parent: &Option<Box<KaitaiStruct>>,
                                                    _root: &Option<Box<KaitaiStruct>>)
                                                    -> Result<Self>
                                where Self: Sized {
                                let mut s = Self {
            firstObj: ,
            secondVal: 0,
            errCast: ,
            opcodes: Vec<>*,
            code: 0,
            body: ,
            value: 0,
            value: String,
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
        self.opcodes = [];
        $i = 0;
        while (!stream->isEof()) {
            self.opcodes[] = new switch_cast::opcode(stream, $this, _root);
            $i++;
        }

        Ok(())
    }
    public function firstObj() {
        if (self.firstObj !== null)
            return self.firstObj;
        self.firstObj = $this->opcodes()[0]->body();
        return self.firstObj;
    }
    public function secondVal() {
        if (self.secondVal !== null)
            return self.secondVal;
        self.secondVal = $this->opcodes()[1]->body()->value();
        return self.secondVal;
    }
    public function errCast() {
        if (self.errCast !== null)
            return self.errCast;
        self.errCast = $this->opcodes()[2]->body();
        return self.errCast;
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
        self.code = stream.read_u1()?;
        switch ($this->code()) {
            case 73:
                self.body = new switch_cast::intval(stream, $this, _root);
                break;
            case 83:
                self.body = new switch_cast::strval(stream, $this, _root);
                break;
        }

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
        self.value = stream.read_u1()?;

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
        self.value = &mut S::bytesToStr(stream->readBytesTerm(0, false, true, true), "ASCII");

        Ok(())
    }
}
