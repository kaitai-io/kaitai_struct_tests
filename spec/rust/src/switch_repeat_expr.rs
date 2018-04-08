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

pub struct SwitchRepeatExpr {
pub struct One {
pub struct Two {
    pub code: u8,
    pub size: u32,
    pub body: Vec<>*,
    pub _raw_body: Vec<String>*,
    pub first: String,
    pub second: String,
}

impl KaitaiStruct for SwitchRepeatExpr {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for One {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
                }

                impl KaitaiStruct for Two {
                    fn new<S: KaitaiStream>(stream: &mut S,
                                            _parent: &Option<Box<KaitaiStruct>>,
                                            _root: &Option<Box<KaitaiStruct>>)
                                            -> Result<Self>
                        where Self: Sized {
                        let mut s = Self {
            code: 0,
            size: 0,
            body: Vec<>*,
            _raw_body: Vec<String>*,
            first: String,
            second: String,
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
        self.size = stream.read_u4le()?;
        self._raw_body = [];
        self.body = [];
        $n = 1;
        for ($i = 0; $i < $n; $i++) {
            switch ($this->code()) {
                case 17:
                    self._raw_body[] = stream->readBytes($this->size());
                    $io = new &mut S(end(self._raw_body));
                    self.body[] = new switch_repeat_expr::one($io, $this, _root);
                    break;
                case 34:
                    self._raw_body[] = stream->readBytes($this->size());
                    $io = new &mut S(end(self._raw_body));
                    self.body[] = new switch_repeat_expr::two($io, $this, _root);
                    break;
                default:
                    self.body[] = stream->readBytes($this->size());
                    break;
            }
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
        self.first = stream->readBytesFull();

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
        self.second = stream->readBytesFull();

        Ok(())
    }
}
