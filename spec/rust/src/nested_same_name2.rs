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

pub struct NestedSameName2 {
pub struct Main {
pub struct FooObj {
pub struct DummyObj {
pub struct FooObj {
    pub version: u32,
    pub mainData: ,
    pub dummy: ,
    pub mainSize: i32,
    pub foo: ,
    pub data1: String,
    pub dummySize: i32,
    pub foo: ,
    pub data2: String,
}

impl KaitaiStruct for NestedSameName2 {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for Main {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
                }

                impl KaitaiStruct for FooObj {
                    fn new<S: KaitaiStream>(stream: &mut S,
                                            _parent: &Option<Box<KaitaiStruct>>,
                                            _root: &Option<Box<KaitaiStruct>>)
                                            -> Result<Self>
                        where Self: Sized {
                        let mut s = Self {
                        }

                        impl KaitaiStruct for DummyObj {
                            fn new<S: KaitaiStream>(stream: &mut S,
                                                    _parent: &Option<Box<KaitaiStruct>>,
                                                    _root: &Option<Box<KaitaiStruct>>)
                                                    -> Result<Self>
                                where Self: Sized {
                                let mut s = Self {
                                }

                                impl KaitaiStruct for FooObj {
                                    fn new<S: KaitaiStream>(stream: &mut S,
                                                            _parent: &Option<Box<KaitaiStruct>>,
                                                            _root: &Option<Box<KaitaiStruct>>)
                                                            -> Result<Self>
                                        where Self: Sized {
                                        let mut s = Self {
            version: 0,
            mainData: ,
            dummy: ,
            mainSize: 0,
            foo: ,
            data1: String,
            dummySize: 0,
            foo: ,
            data2: String,
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
        self.version = stream.read_u4le()?;
        self.mainData = new nested_same_name2::main(stream, $this, _root);
        self.dummy = new nested_same_name2::dummy_obj(stream, $this, _root);

        Ok(())
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
        self.mainSize = stream.read_s4le()?;
        self.foo = new nested_same_name2::main::foo_obj(stream, $this, _root);

        Ok(())
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
        self.data1 = stream->readBytes(($this->_parent()->mainSize() * 2));

        Ok(())
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
        self.dummySize = stream.read_s4le()?;
        self.foo = new nested_same_name2::dummy_obj::foo_obj(stream, $this, _root);

        Ok(())
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
        self.data2 = stream->readBytes(($this->_parent()->dummySize() * 2));

        Ok(())
    }
}
