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

pub struct EnumIf {
pub struct Operation {
pub struct ArgTuple {
pub struct ArgStr {
pub struct Opcodes {
    pub op1: ,
    pub op2: ,
    pub op3: ,
    pub opcode: ,
    pub argTuple: ,
    pub argStr: ,
    pub num1: u8,
    pub num2: u8,
    pub len: u8,
    pub str: String,
}

impl KaitaiStruct for EnumIf {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for Operation {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
                }

                impl KaitaiStruct for ArgTuple {
                    fn new<S: KaitaiStream>(stream: &mut S,
                                            _parent: &Option<Box<KaitaiStruct>>,
                                            _root: &Option<Box<KaitaiStruct>>)
                                            -> Result<Self>
                        where Self: Sized {
                        let mut s = Self {
                        }

                        impl KaitaiStruct for ArgStr {
                            fn new<S: KaitaiStream>(stream: &mut S,
                                                    _parent: &Option<Box<KaitaiStruct>>,
                                                    _root: &Option<Box<KaitaiStruct>>)
                                                    -> Result<Self>
                                where Self: Sized {
                                let mut s = Self {
            op1: ,
            op2: ,
            op3: ,
            opcode: ,
            argTuple: ,
            argStr: ,
            num1: 0,
            num2: 0,
            len: 0,
            str: String,
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
        self.op1 = new enum_if::operation(stream, $this, _root);
        self.op2 = new enum_if::operation(stream, $this, _root);
        self.op3 = new enum_if::operation(stream, $this, _root);

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
        self.opcode = stream.read_u1()?;
        if ($this->opcode() == enum_if::opcodes::A_TUPLE) {
            self.argTuple = new enum_if::arg_tuple(stream, $this, _root);
        }
        if ($this->opcode() == enum_if::opcodes::A_STRING) {
            self.argStr = new enum_if::arg_str(stream, $this, _root);
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
        self.num1 = stream.read_u1()?;
        self.num2 = stream.read_u1()?;

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
        self.len = stream.read_u1()?;
        self.str = &mut S::bytesToStr(stream->readBytes($this->len()), "UTF-8");

        Ok(())
    }
}
const A_STRING = 83;
const A_TUPLE = 84;
}
