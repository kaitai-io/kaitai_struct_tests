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

pub struct CastNested {
pub struct Opcode {
pub struct Intval {
pub struct Strval {
    pub opcodes0Str: ,
    pub opcodes0StrValue: String,
    pub opcodes1Int: ,
    pub opcodes1IntValue: u8,
    pub opcodes: Vec<>*,
    pub code: u8,
    pub body: ,
    pub value: u8,
    pub value: String,
}

impl KaitaiStruct for CastNested {
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
            opcodes0Str: ,
            opcodes0StrValue: String,
            opcodes1Int: ,
            opcodes1IntValue: 0,
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
            self.opcodes[] = new cast_nested::opcode(stream, $this, _root);
            $i++;
        }

        Ok(())
    }
    public function opcodes0Str() {
        if (self.opcodes0Str !== null)
            return self.opcodes0Str;
        self.opcodes0Str = $this->opcodes()[0]->body();
        return self.opcodes0Str;
    }
    public function opcodes0StrValue() {
        if (self.opcodes0StrValue !== null)
            return self.opcodes0StrValue;
        self.opcodes0StrValue = $this->opcodes()[0]->body()->value();
        return self.opcodes0StrValue;
    }
    public function opcodes1Int() {
        if (self.opcodes1Int !== null)
            return self.opcodes1Int;
        self.opcodes1Int = $this->opcodes()[1]->body();
        return self.opcodes1Int;
    }
    public function opcodes1IntValue() {
        if (self.opcodes1IntValue !== null)
            return self.opcodes1IntValue;
        self.opcodes1IntValue = $this->opcodes()[1]->body()->value();
        return self.opcodes1IntValue;
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
                self.body = new cast_nested::opcode::intval(stream, $this, _root);
                break;
            case 83:
                self.body = new cast_nested::opcode::strval(stream, $this, _root);
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
