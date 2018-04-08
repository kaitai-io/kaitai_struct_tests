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

pub struct MultipleUse {
pub struct Multi {
pub struct Type1 {
pub struct Type2 {
    pub t1: ,
    pub t2: ,
    pub value: i32,
    pub firstUse: ,
    pub secondUse: ,
}

impl KaitaiStruct for MultipleUse {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for Multi {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
                }

                impl KaitaiStruct for Type1 {
                    fn new<S: KaitaiStream>(stream: &mut S,
                                            _parent: &Option<Box<KaitaiStruct>>,
                                            _root: &Option<Box<KaitaiStruct>>)
                                            -> Result<Self>
                        where Self: Sized {
                        let mut s = Self {
                        }

                        impl KaitaiStruct for Type2 {
                            fn new<S: KaitaiStream>(stream: &mut S,
                                                    _parent: &Option<Box<KaitaiStruct>>,
                                                    _root: &Option<Box<KaitaiStruct>>)
                                                    -> Result<Self>
                                where Self: Sized {
                                let mut s = Self {
            t1: ,
            t2: ,
            value: 0,
            firstUse: ,
            secondUse: ,
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
        self.t1 = new multiple_use::type_1(stream, $this, _root);
        self.t2 = new multiple_use::type_2(stream, $this, _root);

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
        self.value = stream.read_s4le()?;

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
        self.firstUse = new multiple_use::multi(stream, $this, _root);

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
    public function secondUse() {
        if (self.secondUse !== null)
            return self.secondUse;
        $_pos = stream->pos();
        stream->seek(0);
        self.secondUse = new multiple_use::multi(stream, $this, _root);
        stream->seek($_pos);
        return self.secondUse;
    }
}
