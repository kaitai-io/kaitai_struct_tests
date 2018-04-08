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

pub struct InstanceUserArray {
pub struct Entry {
    pub userEntries: Vec<>*,
    pub ofs: u32,
    pub entrySize: u32,
    pub qtyEntries: u32,
    pub _raw_userEntries: Vec<String>*,
    pub word1: u16,
    pub word2: u16,
}

impl KaitaiStruct for InstanceUserArray {
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
            userEntries: Vec<>*,
            ofs: 0,
            entrySize: 0,
            qtyEntries: 0,
            _raw_userEntries: Vec<String>*,
            word1: 0,
            word2: 0,
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
        self.ofs = stream.read_u4le()?;
        self.entrySize = stream.read_u4le()?;
        self.qtyEntries = stream.read_u4le()?;

        Ok(())
    }
    public function userEntries() {
        if (self.userEntries !== null)
            return self.userEntries;
        if ($this->ofs() > 0) {
            $_pos = stream->pos();
            stream->seek($this->ofs());
            self._raw_userEntries = [];
            self.userEntries = [];
            $n = $this->qtyEntries();
            for ($i = 0; $i < $n; $i++) {
                self._raw_userEntries[] = stream->readBytes($this->entrySize());
                $io = new &mut S(end(self._raw_userEntries));
                self.userEntries[] = new instance_user_array::entry($io, $this, _root);
            }
            stream->seek($_pos);
        }
        return self.userEntries;
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
        self.word1 = stream.read_u2le()?;
        self.word2 = stream.read_u2le()?;

        Ok(())
    }
}
