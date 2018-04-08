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

pub struct ProcessCoerceBytes {
pub struct Record {
    pub records: Vec<>*,
    pub buf: String,
    pub flag: u8,
    pub bufUnproc: String,
    pub bufProc: String,
    pub _raw_bufProc: String,
}

impl KaitaiStruct for ProcessCoerceBytes {
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
            records: Vec<>*,
            buf: String,
            flag: 0,
            bufUnproc: String,
            bufProc: String,
            _raw_bufProc: String,
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
            self.records[] = new process_coerce_bytes::record(stream, $this, _root);
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
            self.bufUnproc = stream->readBytes(4);
        }
        if ($this->flag() != 0) {
            self._raw_bufProc = stream->readBytes(4);
            self.bufProc = &mut S::processXorOne(self._raw_bufProc, 170);
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
