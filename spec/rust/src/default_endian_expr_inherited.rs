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

pub struct DefaultEndianExprInherited {
pub struct Doc {
pub struct MainObj {
pub struct SubObj {
pub struct SubsubObj {
    pub docs: Vec<>*,
    pub indicator: String,
    pub main: ,
    pub insides: ,
    pub someInt: u32,
    pub more: ,
    pub someInst: u32,
    pub someInt1: u16,
    pub someInt2: u16,
}

impl KaitaiStruct for DefaultEndianExprInherited {
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

                        impl KaitaiStruct for SubObj {
                            fn new<S: KaitaiStream>(stream: &mut S,
                                                    _parent: &Option<Box<KaitaiStruct>>,
                                                    _root: &Option<Box<KaitaiStruct>>)
                                                    -> Result<Self>
                                where Self: Sized {
                                let mut s = Self {
                                }

                                impl KaitaiStruct for SubsubObj {
                                    fn new<S: KaitaiStream>(stream: &mut S,
                                                            _parent: &Option<Box<KaitaiStruct>>,
                                                            _root: &Option<Box<KaitaiStruct>>)
                                                            -> Result<Self>
                                        where Self: Sized {
                                        let mut s = Self {
            docs: Vec<>*,
            indicator: String,
            main: ,
            insides: ,
            someInt: 0,
            more: ,
            someInst: 0,
            someInt1: 0,
            someInt2: 0,
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
            self.docs[] = new default_endian_expr_inherited::doc(stream, $this, _root);
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
        self.main = new default_endian_expr_inherited::doc::main_obj(stream, $this, _root);

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
        self.insides = new default_endian_expr_inherited::doc::main_obj::sub_obj(stream, $this, _root, self._is_le);

        Ok(())
    }

    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S,
                             _parent: &Option<Box<KaitaiStruct>>,
                             _root: &Option<Box<KaitaiStruct>>)
                             -> Result<()>
        where Self: Sized {
        self.insides = new default_endian_expr_inherited::doc::main_obj::sub_obj(stream, $this, _root, self._is_le);

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

        Ok(())
    }

    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S,
                             _parent: &Option<Box<KaitaiStruct>>,
                             _root: &Option<Box<KaitaiStruct>>)
                             -> Result<()>
        where Self: Sized {
        self.someInt = stream.read_u4le()?;
        self.more = new default_endian_expr_inherited::doc::main_obj::sub_obj::subsub_obj(stream, $this, _root, self._is_le);

        Ok(())
    }

    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S,
                             _parent: &Option<Box<KaitaiStruct>>,
                             _root: &Option<Box<KaitaiStruct>>)
                             -> Result<()>
        where Self: Sized {
        self.someInt = stream.read_u4be()?;
        self.more = new default_endian_expr_inherited::doc::main_obj::sub_obj::subsub_obj(stream, $this, _root, self._is_le);

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

        Ok(())
    }

    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S,
                             _parent: &Option<Box<KaitaiStruct>>,
                             _root: &Option<Box<KaitaiStruct>>)
                             -> Result<()>
        where Self: Sized {
        self.someInt1 = stream.read_u2le()?;
        self.someInt2 = stream.read_u2le()?;

        Ok(())
    }

    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S,
                             _parent: &Option<Box<KaitaiStruct>>,
                             _root: &Option<Box<KaitaiStruct>>)
                             -> Result<()>
        where Self: Sized {
        self.someInt1 = stream.read_u2be()?;
        self.someInt2 = stream.read_u2be()?;

        Ok(())
    }
    public function someInst() {
        if (self.someInst !== null)
            return self.someInst;
        $_pos = stream->pos();
        stream->seek(2);
        if ($this->_m__is_le) {
            self.someInst = stream.read_u4le()?;
        } else {
            self.someInst = stream.read_u4be()?;
        }
        stream->seek($_pos);
        return self.someInst;
    }
}
