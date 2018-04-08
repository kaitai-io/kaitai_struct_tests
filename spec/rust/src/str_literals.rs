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

pub struct StrLiterals {
    pub octalEatup2: String,
    pub backslashes: String,
    pub octalEatup: String,
    pub doubleQuotes: String,
    pub complexStr: String,
}

impl KaitaiStruct for StrLiterals {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            octalEatup2: String,
            backslashes: String,
            octalEatup: String,
            doubleQuotes: String,
            complexStr: String,
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
    public function octalEatup2() {
        if (self.octalEatup2 !== null)
            return self.octalEatup2;
        self.octalEatup2 = "\0022";
        return self.octalEatup2;
    }
    public function backslashes() {
        if (self.backslashes !== null)
            return self.backslashes;
        self.backslashes = "\\\\\\";
        return self.backslashes;
    }
    public function octalEatup() {
        if (self.octalEatup !== null)
            return self.octalEatup;
        self.octalEatup = "\00022";
        return self.octalEatup;
    }
    public function doubleQuotes() {
        if (self.doubleQuotes !== null)
            return self.doubleQuotes;
        self.doubleQuotes = "\"\"\"";
        return self.doubleQuotes;
    }
    public function complexStr() {
        if (self.complexStr !== null)
            return self.complexStr;
        self.complexStr = "\000\001\002\007\010\n\r\t\v\f\e=\007\n\$\u{263b}";
        return self.complexStr;
    }
}
