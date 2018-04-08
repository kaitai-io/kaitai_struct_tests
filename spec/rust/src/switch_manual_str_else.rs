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

pub struct SwitchManualStrElse {
pub struct Opcode {
pub struct Intval {
pub struct Strval {
pub struct Noneval {
    pub opcodes: Vec<>*,
    pub code: String,
    pub body: ,
    pub value: u8,
    pub value: String,
    pub filler: u32,
}

impl KaitaiStruct for SwitchManualStrElse {
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
                                }

                                impl KaitaiStruct for Noneval {
                                    fn new<S: KaitaiStream>(stream: &mut S,
                                                            _parent: &Option<Box<KaitaiStruct>>,
                                                            _root: &Option<Box<KaitaiStruct>>)
                                                            -> Result<Self>
                                        where Self: Sized {
                                        let mut s = Self {
            opcodes: Vec<>*,
            code: String,
            body: ,
            value: 0,
            value: String,
            filler: 0,
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
            self.opcodes[] = new switch_manual_str_else::opcode(stream, $this, _root);
            $i++;
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
        self.code = &mut S::bytesToStr(stream->readBytes(1), "ASCII");
        switch ($this->code()) {
            case "I":
                self.body = new switch_manual_str_else::opcode::intval(stream, $this, _root);
                break;
            case "S":
                self.body = new switch_manual_str_else::opcode::strval(stream, $this, _root);
                break;
            default:
                self.body = new switch_manual_str_else::opcode::noneval(stream, $this, _root);
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
        self.filler = stream.read_u4le()?;

        Ok(())
    }
}
