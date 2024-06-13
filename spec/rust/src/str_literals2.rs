// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct StrLiterals2 {
    pub dollar1: Option<String>,
    pub dollar2: Option<String>,
    pub hash: Option<String>,
    pub atSign: Option<String>,
}

impl KaitaiStruct for StrLiterals2 {
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

impl StrLiterals2 {
    fn dollar1(&mut self) -> String {
        if let Some(x) = self.dollar1 {
            return x;
        }

        self.dollar1 = "$foo";
        return self.dollar1;
    }
    fn dollar2(&mut self) -> String {
        if let Some(x) = self.dollar2 {
            return x;
        }

        self.dollar2 = "${foo}";
        return self.dollar2;
    }
    fn hash(&mut self) -> String {
        if let Some(x) = self.hash {
            return x;
        }

        self.hash = "#{foo}";
        return self.hash;
    }
    fn atSign(&mut self) -> String {
        if let Some(x) = self.atSign {
            return x;
        }

        self.atSign = "@foo";
        return self.atSign;
    }
}
