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

pub struct NestedTypes {
pub struct SubtypeA {
pub struct SubtypeC {
pub struct SubtypeB {
    pub one: ,
    pub two: ,
    pub typedAtRoot: ,
    pub typedHere: ,
    pub valueC: i8,
    pub valueB: i8,
}

impl KaitaiStruct for NestedTypes {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for SubtypeA {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
                }

                impl KaitaiStruct for SubtypeC {
                    fn new<S: KaitaiStream>(stream: &mut S,
                                            _parent: &Option<Box<KaitaiStruct>>,
                                            _root: &Option<Box<KaitaiStruct>>)
                                            -> Result<Self>
                        where Self: Sized {
                        let mut s = Self {
                        }

                        impl KaitaiStruct for SubtypeB {
                            fn new<S: KaitaiStream>(stream: &mut S,
                                                    _parent: &Option<Box<KaitaiStruct>>,
                                                    _root: &Option<Box<KaitaiStruct>>)
                                                    -> Result<Self>
                                where Self: Sized {
                                let mut s = Self {
            one: ,
            two: ,
            typedAtRoot: ,
            typedHere: ,
            valueC: 0,
            valueB: 0,
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
        self.one = new nested_types::subtype_a(stream, $this, _root);
        self.two = new nested_types::subtype_b(stream, $this, _root);

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
        self.typedAtRoot = new nested_types::subtype_b(stream, $this, _root);
        self.typedHere = new nested_types::subtype_a::subtype_c(stream, $this, _root);

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
        self.valueC = stream.read_s1()?;

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
        self.valueB = stream.read_s1()?;

        Ok(())
    }
}
