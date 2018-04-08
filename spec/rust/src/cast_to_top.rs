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

pub struct CastToTop {
    pub header: ,
    pub headerCasted: ,
    pub code: u8,
}

impl KaitaiStruct for CastToTop {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            header: ,
            headerCasted: ,
            code: 0,
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
        self.code = stream.read_u1()?;

        Ok(())
    }
    public function header() {
        if (self.header !== null)
            return self.header;
        $_pos = stream->pos();
        stream->seek(1);
        self.header = new cast_to_top(stream);
        stream->seek($_pos);
        return self.header;
    }
    public function headerCasted() {
        if (self.headerCasted !== null)
            return self.headerCasted;
        self.headerCasted = $this->header();
        return self.headerCasted;
    }
}
