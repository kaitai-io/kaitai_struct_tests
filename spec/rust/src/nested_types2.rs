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

pub struct NestedTypes2 {
pub struct SubtypeA {
pub struct SubtypeC {
pub struct SubtypeD {
pub struct SubtypeCc {
pub struct SubtypeB {
    pub one: ,
    pub two: ,
    pub typedAtRoot: ,
    pub typedHere1: ,
    pub typedHere2: ,
    pub valueC: i8,
    pub typedHere: ,
    pub typedParent: ,
    pub typedRoot: ,
    pub valueD: i8,
    pub valueCc: i8,
    pub valueB: i8,
}

impl KaitaiStruct for NestedTypes2 {
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

                        impl KaitaiStruct for SubtypeD {
                            fn new<S: KaitaiStream>(stream: &mut S,
                                                    _parent: &Option<Box<KaitaiStruct>>,
                                                    _root: &Option<Box<KaitaiStruct>>)
                                                    -> Result<Self>
                                where Self: Sized {
                                let mut s = Self {
                                }

                                impl KaitaiStruct for SubtypeCc {
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
            typedHere1: ,
            typedHere2: ,
            valueC: 0,
            typedHere: ,
            typedParent: ,
            typedRoot: ,
            valueD: 0,
            valueCc: 0,
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
        self.one = new nested_types2::subtype_a(stream, $this, _root);
        self.two = new nested_types2::subtype_b(stream, $this, _root);

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
        self.typedAtRoot = new nested_types2::subtype_b(stream, $this, _root);
        self.typedHere1 = new nested_types2::subtype_a::subtype_c(stream, $this, _root);
        self.typedHere2 = new nested_types2::subtype_a::subtype_cc(stream, $this, _root);

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
        self.typedHere = new nested_types2::subtype_a::subtype_c::subtype_d(stream, $this, _root);
        self.typedParent = new nested_types2::subtype_a::subtype_cc(stream, $this, _root);
        self.typedRoot = new nested_types2::subtype_b(stream, $this, _root);

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
        self.valueD = stream.read_s1()?;

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
        self.valueCc = stream.read_s1()?;

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
