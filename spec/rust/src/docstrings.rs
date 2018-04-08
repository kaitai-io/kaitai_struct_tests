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

pub struct Docstrings {
pub struct ComplexSubtype {
    pub two: u8,
    pub three: i8,
    pub one: u8,
}

impl KaitaiStruct for Docstrings {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for ComplexSubtype {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
            two: 0,
            three: 0,
            one: 0,

/**
 * One-liner description of a type.
 */
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
        self.one = stream.read_u1()?;

        Ok(())
    }

    /**
     * Another description for parse instance "two"
     */
    public function two() {
        if (self.two !== null)
            return self.two;
        $_pos = stream->pos();
        stream->seek(0);
        self.two = stream.read_u1()?;
        stream->seek($_pos);
        return self.two;
    }

    /**
     * And yet another one for value instance "three"
     */
    public function three() {
        if (self.three !== null)
            return self.three;
        self.three = 66;
        return self.three;
    }

    /**
     * A pretty verbose description for sequence attribute "one"
     */
}

/**
 * This subtype is never used, yet has a very long description
 * that spans multiple lines. It should be formatted accordingly,
 * even in languages that have single-line comments for
 * docstrings. Actually, there's even a MarkDown-style list here
 * with several bullets:
 * 
 * * one
 * * two
 * * three
 * 
 * And the text continues after that. Here's a MarkDown-style link:
 * [woohoo](http://example.com) - one day it will be supported as
 * well.
 */
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
