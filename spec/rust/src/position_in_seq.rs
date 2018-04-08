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

pub struct PositionInSeq {
pub struct HeaderObj {
    pub header: ,
    pub numbers: Vec<u8>*,
    pub qtyNumbers: u32,
}

impl KaitaiStruct for PositionInSeq {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for HeaderObj {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
            header: ,
            numbers: Vec<u8>*,
            qtyNumbers: 0,
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
        self.numbers = [];
        $n = $this->header()->qtyNumbers();
        for ($i = 0; $i < $n; $i++) {
            self.numbers[] = stream.read_u1()?;
        }

        Ok(())
    }
    public function header() {
        if (self.header !== null)
            return self.header;
        $_pos = stream->pos();
        stream->seek(16);
        self.header = new position_in_seq::header_obj(stream, $this, _root);
        stream->seek($_pos);
        return self.header;
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
        self.qtyNumbers = stream.read_u4le()?;

        Ok(())
    }
}
