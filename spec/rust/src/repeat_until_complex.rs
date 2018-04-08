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

pub struct RepeatUntilComplex {
pub struct TypeU1 {
pub struct TypeU2 {
    pub first: Vec<>*,
    pub second: Vec<>*,
    pub third: Vec<u8>*,
    pub count: u8,
    pub values: Vec<u8>*,
    pub count: u16,
    pub values: Vec<u16>*,
}

impl KaitaiStruct for RepeatUntilComplex {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for TypeU1 {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
                }

                impl KaitaiStruct for TypeU2 {
                    fn new<S: KaitaiStream>(stream: &mut S,
                                            _parent: &Option<Box<KaitaiStruct>>,
                                            _root: &Option<Box<KaitaiStruct>>)
                                            -> Result<Self>
                        where Self: Sized {
                        let mut s = Self {
            first: Vec<>*,
            second: Vec<>*,
            third: Vec<u8>*,
            count: 0,
            values: Vec<u8>*,
            count: 0,
            values: Vec<u16>*,
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
        self.first = [];
        $i = 0;
        do {
            $_ = new repeat_until_complex::type_u1(stream, $this, _root);
            self.first[] = $_;
            $i++;
        } while (!($_->count() == 0));
        self.second = [];
        $i = 0;
        do {
            $_ = new repeat_until_complex::type_u2(stream, $this, _root);
            self.second[] = $_;
            $i++;
        } while (!($_->count() == 0));
        self.third = [];
        $i = 0;
        do {
            $_ = stream.read_u1()?;
            self.third[] = $_;
            $i++;
        } while (!($_ == 0));

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
        self.count = stream.read_u1()?;
        self.values = [];
        $n = $this->count();
        for ($i = 0; $i < $n; $i++) {
            self.values[] = stream.read_u1()?;
        }

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
        self.count = stream.read_u2le()?;
        self.values = [];
        $n = $this->count();
        for ($i = 0; $i < $n; $i++) {
            self.values[] = stream.read_u2le()?;
        }

        Ok(())
    }
}
