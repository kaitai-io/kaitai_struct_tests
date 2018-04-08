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

pub struct Expr0 {
    pub mustBeF7: i32,
    pub mustBeAbc123: String,
    pub lenOf1: u16,
}

impl KaitaiStruct for Expr0 {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            mustBeF7: i32,
            mustBeAbc123: String,
            lenOf1: 0,
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
        self.lenOf1 = stream.read_u2le()?;

        Ok(())
    }
    public function mustBeF7() {
        if (self.mustBeF7 !== null)
            return self.mustBeF7;
        self.mustBeF7 = (7 + 240);
        return self.mustBeF7;
    }
    public function mustBeAbc123() {
        if (self.mustBeAbc123 !== null)
            return self.mustBeAbc123;
        self.mustBeAbc123 = "abc" . "123";
        return self.mustBeAbc123;
    }
}
