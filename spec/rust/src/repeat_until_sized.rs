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

pub struct RepeatUntilSized {
pub struct Record {
    pub records: Vec<>*,
    pub _raw_records: Vec<String>*,
    pub marker: u8,
    pub body: u32,
}

impl KaitaiStruct for RepeatUntilSized {
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
            _raw_records: Vec<String>*,
            marker: 0,
            body: 0,
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
        self._raw_records = [];
        self.records = [];
        $i = 0;
        do {
            $_buf = stream->readBytes(5);
            self._raw_records[] = $_buf;
            $io = new &mut S($_buf);
            $_ = new repeat_until_sized::record($io, $this, _root);
            self.records[] = $_;
            $i++;
        } while (!($_->marker() == 170));

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
        self.marker = stream.read_u1()?;
        self.body = stream.read_u4le()?;

        Ok(())
    }
}
