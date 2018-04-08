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

pub struct IfInstances {
    pub neverHappens: u8,
}

impl KaitaiStruct for IfInstances {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            neverHappens: 0,
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

        Ok(())
    }
    public function neverHappens() {
        if (self.neverHappens !== null)
            return self.neverHappens;
        if (false) {
            $_pos = stream->pos();
            stream->seek(100500);
            self.neverHappens = stream.read_u1()?;
            stream->seek($_pos);
        }
        return self.neverHappens;
    }
}
