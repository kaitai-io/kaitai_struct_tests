// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::io::Cursor;
use std::vec::Vec;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct StrLiterals {
    pub octalEatup2: Option<String>,
    pub backslashes: Option<String>,
    pub octalEatup: Option<String>,
    pub doubleQuotes: Option<String>,
    pub complexStr: Option<String>,
}

impl KaitaiStruct for StrLiterals {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s: Self = Default::default();

        s.stream = stream;
        s.read(stream, _parent, _root)?;

        Ok(s)
    }


    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S,
                             _parent: &Option<Box<KaitaiStruct>>,
                             _root: &Option<Box<KaitaiStruct>>)
                             -> Result<()>
        where Self: Sized {
    }
}

impl StrLiterals {
    fn octalEatup2(&mut self) -> String {
        if let Some(x) = self.octalEatup2 {
            return x;
        }

        self.octalEatup2 = "\0022";
        return self.octalEatup2;
    }
    fn backslashes(&mut self) -> String {
        if let Some(x) = self.backslashes {
            return x;
        }

        self.backslashes = "\\\\\\";
        return self.backslashes;
    }
    fn octalEatup(&mut self) -> String {
        if let Some(x) = self.octalEatup {
            return x;
        }

        self.octalEatup = "\00022";
        return self.octalEatup;
    }
    fn doubleQuotes(&mut self) -> String {
        if let Some(x) = self.doubleQuotes {
            return x;
        }

        self.doubleQuotes = "\"\"\"";
        return self.doubleQuotes;
    }
    fn complexStr(&mut self) -> String {
        if let Some(x) = self.complexStr {
            return x;
        }

        self.complexStr = "\000\001\002\007\010\n\r\t\013\014\033=\007\n$\u{263b}";
        return self.complexStr;
    }
}
