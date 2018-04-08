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

pub struct JsSignedRightShift {
    pub shouldBe40000000: i32,
    pub shouldBeA00000: i32,
}

impl KaitaiStruct for JsSignedRightShift {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            shouldBe40000000: i32,
            shouldBeA00000: i32,
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
    public function shouldBe40000000() {
        if (self.shouldBe40000000 !== null)
            return self.shouldBe40000000;
        self.shouldBe40000000 = (2147483648 >> 1);
        return self.shouldBe40000000;
    }
    public function shouldBeA00000() {
        if (self.shouldBeA00000 !== null)
            return self.shouldBeA00000;
        self.shouldBeA00000 = (2684354560 >> 8);
        return self.shouldBeA00000;
    }
}
