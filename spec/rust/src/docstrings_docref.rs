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

pub struct DocstringsDocref {
    pub foo: bool,
    pub parseInst: u8,
    pub one: u8,
    pub two: u8,
    pub three: u8,
}

impl KaitaiStruct for DocstringsDocref {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            foo: bool,
            parseInst: 0,
            one: 0,
            two: 0,
            three: 0,

/**
 * Another one-liner
 */
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
        self.one = stream.read_u1()?;
        self.two = stream.read_u1()?;
        self.three = stream.read_u1()?;

        Ok(())
    }
    public function foo() {
        if (self.foo !== null)
            return self.foo;
        self.foo = true;
        return self.foo;
    }
    public function parseInst() {
        if (self.parseInst !== null)
            return self.parseInst;
        $_pos = stream->pos();
        stream->seek(0);
        self.parseInst = stream.read_u1()?;
        stream->seek($_pos);
        return self.parseInst;
    }

    /**
     * Both doc and doc-ref are defined
     */
}
