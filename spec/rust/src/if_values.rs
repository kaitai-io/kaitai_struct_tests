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

pub struct IfValues {
pub struct Code {
    pub codes: Vec<>*,
    pub halfOpcode: i32,
    pub opcode: u8,
}

impl KaitaiStruct for IfValues {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for Code {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
            codes: Vec<>*,
            halfOpcode: i32,
            opcode: 0,
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
        self.codes = [];
        $n = 3;
        for ($i = 0; $i < $n; $i++) {
            self.codes[] = new if_values::code(stream, $this, _root);
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
        self.opcode = stream.read_u1()?;

        Ok(())
    }
    public function halfOpcode() {
        if (self.halfOpcode !== null)
            return self.halfOpcode;
        if (&mut S::mod($this->opcode(), 2) == 0) {
            self.halfOpcode = intval($this->opcode() / 2);
        }
        return self.halfOpcode;
    }
}
