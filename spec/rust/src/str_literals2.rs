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

pub struct StrLiterals2 {
    pub dollar1: String,
    pub dollar2: String,
    pub hash: String,
    pub atSign: String,
}

impl KaitaiStruct for StrLiterals2 {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            dollar1: String,
            dollar2: String,
            hash: String,
            atSign: String,
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
    public function dollar1() {
        if (self.dollar1 !== null)
            return self.dollar1;
        self.dollar1 = "\$foo";
        return self.dollar1;
    }
    public function dollar2() {
        if (self.dollar2 !== null)
            return self.dollar2;
        self.dollar2 = "\${foo}";
        return self.dollar2;
    }
    public function hash() {
        if (self.hash !== null)
            return self.hash;
        self.hash = "#{foo}";
        return self.hash;
    }
    public function atSign() {
        if (self.atSign !== null)
            return self.atSign;
        self.atSign = "@foo";
        return self.atSign;
    }
}
