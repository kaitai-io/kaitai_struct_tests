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

pub struct DefaultEndianExprIsLe {
pub struct Doc {
pub struct MainObj {
    pub docs: Vec<>*,
    pub indicator: String,
    pub main: ,
    pub someInt: u32,
    pub someIntBe: u16,
    pub someIntLe: u16,
}

impl KaitaiStruct for DefaultEndianExprIsLe {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for Doc {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
                }

                impl KaitaiStruct for MainObj {
                    fn new<S: KaitaiStream>(stream: &mut S,
                                            _parent: &Option<Box<KaitaiStruct>>,
                                            _root: &Option<Box<KaitaiStruct>>)
                                            -> Result<Self>
                        where Self: Sized {
                        let mut s = Self {
            docs: Vec<>*,
            indicator: String,
            main: ,
            someInt: 0,
            someIntBe: 0,
            someIntLe: 0,
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
        self.docs = [];
        $i = 0;
        while (!stream->isEof()) {
            self.docs[] = new default_endian_expr_is_le::doc(stream, $this, _root);
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
        self.indicator = stream->readBytes(2);
        self.main = new default_endian_expr_is_le::doc::main_obj(stream, $this, _root);

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
        switch ($this->_parent()->indicator()) {
            case "\x49\x49":
                self._is_le = true;
                break;
            default:
                self._is_le = false;
                break;
        }

        Ok(())
    }

    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S,
                             _parent: &Option<Box<KaitaiStruct>>,
                             _root: &Option<Box<KaitaiStruct>>)
                             -> Result<()>
        where Self: Sized {
        self.someInt = stream.read_u4le()?;
        self.someIntBe = stream.read_u2be()?;
        self.someIntLe = stream.read_u2le()?;

        Ok(())
    }

    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S,
                             _parent: &Option<Box<KaitaiStruct>>,
                             _root: &Option<Box<KaitaiStruct>>)
                             -> Result<()>
        where Self: Sized {
        self.someInt = stream.read_u4be()?;
        self.someIntBe = stream.read_u2be()?;
        self.someIntLe = stream.read_u2le()?;

        Ok(())
    }
}
