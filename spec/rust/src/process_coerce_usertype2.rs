// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;
use std::vec::Vec;
use std::io::Cursor;

#[derive(Default)]
pub struct ProcessCoerceUsertype2 {
    pub records: Vec<Box<ProcessCoerceUsertype2__Record>>,
}

impl KaitaiStruct for ProcessCoerceUsertype2 {
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
            self.records.push(Box::new(ProcessCoerceUsertype2__Record::new(self.stream, self, _root)?));
        }
    }
}

impl ProcessCoerceUsertype2 {
}
#[derive(Default)]
pub struct ProcessCoerceUsertype2__Record {
    pub flag: u8,
    pub bufUnproc: Box<ProcessCoerceUsertype2__Foo>,
    pub bufProc: Box<ProcessCoerceUsertype2__Foo>,
    pub _raw_bufProc: Vec<u8>,
    pub _raw__raw_bufProc: Vec<u8>,
    pub buf: Option<Box<ProcessCoerceUsertype2__Foo>>,
}

impl KaitaiStruct for ProcessCoerceUsertype2__Record {
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
            self.bufUnproc = Box::new(ProcessCoerceUsertype2__Foo::new(self.stream, self, _root)?);
        }
        if self.flag != 0 {
            self._raw__raw_bufProc = self.stream.read_bytes(4)?;
            self._raw_bufProc = &mut S::processXorOne(self._raw__raw_bufProc, 170);
            let mut io = Cursor::new(self._raw_bufProc);
            self.bufProc = Box::new(ProcessCoerceUsertype2__Foo::new(self.stream, self, _root)?);
        }
    }
}

impl ProcessCoerceUsertype2__Record {
    fn buf(&mut self) -> Box<ProcessCoerceUsertype2__Foo> {
        if let Some(x) = self.buf {
            return x;
        }

        self.buf = if self.flag == 0 { self.buf_unproc } else { self.buf_proc};
        return self.buf;
    }
}
#[derive(Default)]
pub struct ProcessCoerceUsertype2__Foo {
    pub value: u32,
}

impl KaitaiStruct for ProcessCoerceUsertype2__Foo {
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
        self.value = self.stream.read_u4le()?;
    }
}

impl ProcessCoerceUsertype2__Foo {
}
