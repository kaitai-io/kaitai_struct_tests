// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::io::Cursor;
use std::vec::Vec;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;
use my_custom_fx::MyCustomFx;
use nested::deeply::CustomFx;

#[derive(Default)]
pub struct ProcessCustom {
    pub buf1: Vec<u8>,
    pub buf2: Vec<u8>,
    pub key: u8,
    pub buf3: Vec<u8>,
    pub _raw_buf1: Vec<u8>,
    pub _raw_buf2: Vec<u8>,
    pub _raw_buf3: Vec<u8>,
}

impl KaitaiStruct for ProcessCustom {
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
        self._raw_buf1 = self.stream.read_bytes(5)?;
        let _process = MyCustomFx::new(7, true, vec!([0x20, 0x30, 0x40]));
        self.buf1 = _process.decode(self._raw_buf1);
        self._raw_buf2 = self.stream.read_bytes(5)?;
        let _process = nested::deeply::CustomFx::new(7);
        self.buf2 = _process.decode(self._raw_buf2);
        self.key = self.stream.read_u1()?;
        self._raw_buf3 = self.stream.read_bytes(5)?;
        let _process = MyCustomFx::new(self.key, false, vec!([0x0]));
        self.buf3 = _process.decode(self._raw_buf3);
    }
}

impl ProcessCustom {
}
