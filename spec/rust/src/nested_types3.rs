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

pub struct NestedTypes3 {
pub struct SubtypeA {
pub struct SubtypeC {
pub struct SubtypeD {
pub struct SubtypeCc {
pub struct SubtypeB {
    pub aCc: ,
    pub aCD: ,
    pub b: ,
    pub valueD: i8,
    pub valueCc: i8,
    pub valueB: i8,
    pub aCc: ,
    pub aCD: ,
}

impl KaitaiStruct for NestedTypes3 {
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
            aCc: ,
            aCD: ,
            b: ,
            valueD: 0,
            valueCc: 0,
            valueB: 0,
            aCc: ,
            aCD: ,
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
        self.aCc = new nested_types3::subtype_a::subtype_cc(stream, $this, _root);
        self.aCD = new nested_types3::subtype_a::subtype_c::subtype_d(stream, $this, _root);
        self.b = new nested_types3::subtype_b(stream, $this, _root);

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
        self.aCc = new nested_types3::subtype_a::subtype_cc(stream, $this, _root);
        self.aCD = new nested_types3::subtype_a::subtype_c::subtype_d(stream, $this, _root);

        Ok(())
    }
}
