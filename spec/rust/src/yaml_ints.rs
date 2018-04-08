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

pub struct YamlInts {
    pub testU4Dec: i32,
    pub testU4Hex: i32,
    pub testU8Dec: i32,
    pub testU8Hex: i32,
}

impl KaitaiStruct for YamlInts {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            testU4Dec: i32,
            testU4Hex: i32,
            testU8Dec: i32,
            testU8Hex: i32,
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
    public function testU4Dec() {
        if (self.testU4Dec !== null)
            return self.testU4Dec;
        self.testU4Dec = 4294967295;
        return self.testU4Dec;
    }
    public function testU4Hex() {
        if (self.testU4Hex !== null)
            return self.testU4Hex;
        self.testU4Hex = 4294967295;
        return self.testU4Hex;
    }
    public function testU8Dec() {
        if (self.testU8Dec !== null)
            return self.testU8Dec;
        self.testU8Dec = 18446744073709551615;
        return self.testU8Dec;
    }
    public function testU8Hex() {
        if (self.testU8Hex !== null)
            return self.testU8Hex;
        self.testU8Hex = 18446744073709551615;
        return self.testU8Hex;
    }
}
