// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct JsSignedRightShift {
    pub shouldBe40000000: Option<i32>,
    pub shouldBeA00000: Option<i32>,
}

impl KaitaiStruct for JsSignedRightShift {
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

impl JsSignedRightShift {
    fn shouldBe40000000(&mut self) -> i32 {
        if let Some(x) = self.shouldBe40000000 {
            return x;
        }

        self.shouldBe40000000 = (2147483648 >> 1);
        return self.shouldBe40000000;
    }
    fn shouldBeA00000(&mut self) -> i32 {
        if let Some(x) = self.shouldBeA00000 {
            return x;
        }

        self.shouldBeA00000 = (2684354560 >> 8);
        return self.shouldBeA00000;
    }
}
