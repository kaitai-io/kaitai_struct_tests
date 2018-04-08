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

pub struct InstanceIoUser {
pub struct Entry {
pub struct StringsObj {
    pub qtyEntries: u32,
    pub entries: Vec<>*,
    pub strings: ,
    pub _raw_strings: String,
    pub name: String,
    pub nameOfs: u32,
    pub value: u32,
    pub str: Vec<String>*,
}

impl KaitaiStruct for InstanceIoUser {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for Entry {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
                }

                impl KaitaiStruct for StringsObj {
                    fn new<S: KaitaiStream>(stream: &mut S,
                                            _parent: &Option<Box<KaitaiStruct>>,
                                            _root: &Option<Box<KaitaiStruct>>)
                                            -> Result<Self>
                        where Self: Sized {
                        let mut s = Self {
            qtyEntries: 0,
            entries: Vec<>*,
            strings: ,
            _raw_strings: String,
            name: String,
            nameOfs: 0,
            value: 0,
            str: Vec<String>*,
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
        self.qtyEntries = stream.read_u4le()?;
        self.entries = [];
        $n = $this->qtyEntries();
        for ($i = 0; $i < $n; $i++) {
            self.entries[] = new instance_io_user::entry(stream, $this, _root);
        }
        self._raw_strings = stream->readBytesFull();
        $io = new &mut S(self._raw_strings);
        self.strings = new instance_io_user::strings_obj($io, $this, _root);

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
        self.nameOfs = stream.read_u4le()?;
        self.value = stream.read_u4le()?;

        Ok(())
    }
    public function name() {
        if (self.name !== null)
            return self.name;
        $io = $this->_root()->strings()->_io();
        $_pos = $io->pos();
        $io->seek($this->nameOfs());
        self.name = &mut S::bytesToStr($io->readBytesTerm(0, false, true, true), "UTF-8");
        $io->seek($_pos);
        return self.name;
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
        self.str = [];
        $i = 0;
        while (!stream->isEof()) {
            self.str[] = &mut S::bytesToStr(stream->readBytesTerm(0, false, true, true), "UTF-8");
            $i++;
        }

        Ok(())
    }
}
