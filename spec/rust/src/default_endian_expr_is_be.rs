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

pub struct DefaultEndianExprIsBe {
pub struct Doc {
pub struct MainObj {
pub struct SubMainObj {
    pub docs: Vec<>*,
    pub indicator: String,
    pub main: ,
    pub instInt: u32,
    pub instSub: ,
    pub someInt: u32,
    pub someIntBe: u16,
    pub someIntLe: u16,
    pub foo: u32,
}

impl KaitaiStruct for DefaultEndianExprIsBe {
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
                        }

                        impl KaitaiStruct for SubMainObj {
                            fn new<S: KaitaiStream>(stream: &mut S,
                                                    _parent: &Option<Box<KaitaiStruct>>,
                                                    _root: &Option<Box<KaitaiStruct>>)
                                                    -> Result<Self>
                                where Self: Sized {
                                let mut s = Self {
            docs: Vec<>*,
            indicator: String,
            main: ,
            instInt: 0,
            instSub: ,
            someInt: 0,
            someIntBe: 0,
            someIntLe: 0,
            foo: 0,
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
            self.docs[] = new default_endian_expr_is_be::doc(stream, $this, _root);
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
        self.main = new default_endian_expr_is_be::doc::main_obj(stream, $this, _root);

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
            case "\x4D\x4D":
                self._is_le = false;
                break;
            default:
                self._is_le = true;
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
    public function instInt() {
        if (self.instInt !== null)
            return self.instInt;
        $_pos = stream->pos();
        stream->seek(2);
        if ($this->_m__is_le) {
            self.instInt = stream.read_u4le()?;
        } else {
            self.instInt = stream.read_u4be()?;
        }
        stream->seek($_pos);
        return self.instInt;
    }
    public function instSub() {
        if (self.instSub !== null)
            return self.instSub;
        $_pos = stream->pos();
        stream->seek(2);
        if ($this->_m__is_le) {
            self.instSub = new default_endian_expr_is_be::doc::main_obj::sub_main_obj(stream, $this, _root, self._is_le);
        } else {
            self.instSub = new default_endian_expr_is_be::doc::main_obj::sub_main_obj(stream, $this, _root, self._is_le);
        }
        stream->seek($_pos);
        return self.instSub;
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

        Ok(())
    }

    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S,
                             _parent: &Option<Box<KaitaiStruct>>,
                             _root: &Option<Box<KaitaiStruct>>)
                             -> Result<()>
        where Self: Sized {
        self.foo = stream.read_u4le()?;

        Ok(())
    }

    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S,
                             _parent: &Option<Box<KaitaiStruct>>,
                             _root: &Option<Box<KaitaiStruct>>)
                             -> Result<()>
        where Self: Sized {
        self.foo = stream.read_u4be()?;

        Ok(())
    }
}
