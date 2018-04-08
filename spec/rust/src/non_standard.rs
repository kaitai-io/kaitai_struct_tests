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

pub struct NonStandard {
    pub vi: u8,
    pub pi: u8,
    pub foo: u8,
    pub bar: ,
}

impl KaitaiStruct for NonStandard {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            vi: 0,
            pi: 0,
            foo: 0,
            bar: ,
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
        self.foo = stream.read_u1()?;
        switch ($this->foo()) {
            case 42:
                self.bar = stream.read_u2le()?;
                break;
            case 43:
                self.bar = stream.read_u4le()?;
                break;
        }

        Ok(())
    }
    public function vi() {
        if (self.vi !== null)
            return self.vi;
        self.vi = $this->foo();
        return self.vi;
    }
    public function pi() {
        if (self.pi !== null)
            return self.pi;
        $_pos = stream->pos();
        stream->seek(0);
        self.pi = stream.read_u1()?;
        stream->seek($_pos);
        return self.pi;
    }
}
