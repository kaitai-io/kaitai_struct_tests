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

pub struct OpaqueExternalType02Child {
pub struct OpaqueExternalType02ChildChild {
    pub someMethod: bool,
    pub s1: String,
    pub s2: String,
    pub s3: ,
    pub s3: String,
}

impl KaitaiStruct for OpaqueExternalType02Child {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for OpaqueExternalType02ChildChild {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
            someMethod: bool,
            s1: String,
            s2: String,
            s3: ,
            s3: String,
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
        self.s1 = &mut S::bytesToStr(stream->readBytesTerm(124, false, true, true), "UTF-8");
        self.s2 = &mut S::bytesToStr(stream->readBytesTerm(124, false, false, true), "UTF-8");
        self.s3 = new opaque_external_type_02_child::opaque_external_type_02_child_child(stream, $this, _root);

        Ok(())
    }
    public function someMethod() {
        if (self.someMethod !== null)
            return self.someMethod;
        self.someMethod = true;
        return self.someMethod;
    }
}
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
        if ($this->_root()->someMethod()) {
            self.s3 = &mut S::bytesToStr(stream->readBytesTerm(64, true, true, true), "UTF-8");
        }

        Ok(())
    }
}
