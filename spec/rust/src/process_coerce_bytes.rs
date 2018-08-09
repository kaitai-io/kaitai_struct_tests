// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::io::Cursor;
use std::vec::Vec;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct ProcessCoerceBytes {
    pub records: Vec<Box<ProcessCoerceBytes__Record>>,
}

impl KaitaiStruct for ProcessCoerceBytes {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s: Self = Default::default();

        s.stream = stream;
        s.read(stream, _parent, _root)?;

        Ok(s)
    }


    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S,
                             _parent: &Option<Box<KaitaiStruct>>,
                             _root: &Option<Box<KaitaiStruct>>)
                             -> Result<()>
        where Self: Sized {
        self.records = vec!();
        for i in 0..2 {
            self.records.push(Box::new(ProcessCoerceBytes__Record::new(self.stream, self, _root)?));
        }
    }
}

impl ProcessCoerceBytes {
}
#[derive(Default)]
pub struct ProcessCoerceBytes__Record {
    pub flag: u8,
    pub bufUnproc: Vec<u8>,
    pub bufProc: Vec<u8>,
    pub _raw_bufProc: Vec<u8>,
    pub buf: Option<Vec<u8>>,
}

impl KaitaiStruct for ProcessCoerceBytes__Record {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s: Self = Default::default();

        s.stream = stream;
        s.read(stream, _parent, _root)?;

        Ok(s)
    }


    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S,
                             _parent: &Option<Box<KaitaiStruct>>,
                             _root: &Option<Box<KaitaiStruct>>)
                             -> Result<()>
        where Self: Sized {
        self.flag = self.stream.read_u1()?;
        if self.flag == 0 {
            self.bufUnproc = self.stream.read_bytes(4)?;
        }
        if self.flag != 0 {
            self._raw_bufProc = self.stream.read_bytes(4)?;
            self.bufProc = &mut S::processXorOne(self._raw_bufProc, 170);
        }
    }
}

impl ProcessCoerceBytes__Record {
    fn buf(&mut self) -> Vec<u8> {
        if let Some(x) = self.buf {
            return x;
        }

        self.buf = if self.flag == 0 { self.buf_unproc } else { self.buf_proc};
        return self.buf;
    }
}
