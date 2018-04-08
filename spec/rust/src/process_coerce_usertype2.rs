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

pub struct ProcessCoerceUsertype2 {
pub struct Record {
pub struct Foo {
    pub records: Vec<>*,
    pub buf: ,
    pub flag: u8,
    pub bufUnproc: ,
    pub bufProc: ,
    pub _raw__raw_bufProc: String,
    pub _raw_bufProc: String,
    pub value: u32,
}

impl KaitaiStruct for ProcessCoerceUsertype2 {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for Record {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
                }

                impl KaitaiStruct for Foo {
                    fn new<S: KaitaiStream>(stream: &mut S,
                                            _parent: &Option<Box<KaitaiStruct>>,
                                            _root: &Option<Box<KaitaiStruct>>)
                                            -> Result<Self>
                        where Self: Sized {
                        let mut s = Self {
            records: Vec<>*,
            buf: ,
            flag: 0,
            bufUnproc: ,
            bufProc: ,
            _raw__raw_bufProc: String,
            _raw_bufProc: String,
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
        self.records = [];
        $n = 2;
        for ($i = 0; $i < $n; $i++) {
            self.records[] = new process_coerce_usertype2::record(stream, $this, _root);
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
        self.flag = stream.read_u1()?;
        if ($this->flag() == 0) {
            self.bufUnproc = new process_coerce_usertype2::foo(stream, $this, _root);
        }
        if ($this->flag() != 0) {
            self._raw__raw_bufProc = stream->readBytes(4);
            self._raw_bufProc = &mut S::processXorOne(self._raw__raw_bufProc, 170);
            $io = new &mut S(self._raw_bufProc);
            self.bufProc = new process_coerce_usertype2::foo($io, $this, _root);
        }

        Ok(())
    }
    public function buf() {
        if (self.buf !== null)
            return self.buf;
        self.buf = ($this->flag() == 0 ? $this->bufUnproc() : $this->bufProc());
        return self.buf;
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
        self.value = stream.read_u4le()?;

        Ok(())
    }
}
