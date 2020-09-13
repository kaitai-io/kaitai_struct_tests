// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;


/*
 * One-liner description of a type.
 */
#[derive(Default)]
pub struct Docstrings {
    pub one: u8,
    pub two: Option<u8>,
    pub three: Option<i8>,
}

impl KaitaiStruct for Docstrings {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s: Self = Default::default();

        s.stream = stream;
        s.read(stream, _parent, _root)?;

        Ok(s)
    }


    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S,
                             _parent: &Option<Box<KaitaiStruct>>,
                             _root: &Option<Box<KaitaiStruct>>)
                             -> Result<()>
        where Self: Sized {
        self.one = self.stream.read_u1()?;
    }
}

impl Docstrings {

    /*
     * Another description for parse instance "two"
     */
    fn two(&mut self) -> u8 {
        if let Some(x) = self.two {
            return x;
        }

        let _pos = self.stream.pos();
        self.stream.seek(0);
        self.two = self.stream.read_u1()?;
        self.stream.seek(_pos);
        return self.two;
    }

    /*
     * And yet another one for value instance "three"
     */
    fn three(&mut self) -> i8 {
        if let Some(x) = self.three {
            return x;
        }

        self.three = 66;
        return self.three;
    }

    /*
     * A pretty verbose description for sequence attribute "one"
     */
}

/*
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
#[derive(Default)]
pub struct Docstrings__ComplexSubtype {
}

impl KaitaiStruct for Docstrings__ComplexSubtype {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s: Self = Default::default();

        s.stream = stream;
        s.read(stream, _parent, _root)?;

        Ok(s)
    }


    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S,
                             _parent: &Option<Box<KaitaiStruct>>,
                             _root: &Option<Box<KaitaiStruct>>)
                             -> Result<()>
        where Self: Sized {
    }
}

impl Docstrings__ComplexSubtype {
}
