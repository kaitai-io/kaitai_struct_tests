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

pub struct InstanceStdArray {
    pub entries: Vec<String>*,
    pub ofs: u32,
    pub entrySize: u32,
    pub qtyEntries: u32,
}

impl KaitaiStruct for InstanceStdArray {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            entries: Vec<String>*,
            ofs: 0,
            entrySize: 0,
            qtyEntries: 0,
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
    public function entries() {
        if (self.entries !== null)
            return self.entries;
        $_pos = stream->pos();
        stream->seek($this->ofs());
        self.entries = [];
        $n = $this->qtyEntries();
        for ($i = 0; $i < $n; $i++) {
            self.entries[] = stream->readBytes($this->entrySize());
        }
        stream->seek($_pos);
        return self.entries;
    }
}
