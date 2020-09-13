// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;
use std::vec::Vec;

#[derive(Default)]
pub struct DebugEnumName {
    pub one: Box<DebugEnumName__TestEnum1>,
    pub arrayOfInts: Vec<Box<DebugEnumName__TestEnum2>>,
    pub testType: Box<DebugEnumName__TestSubtype>,
}

impl KaitaiStruct for DebugEnumName {
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
        self.arrayOfInts = vec!();
        for i in 0..1 {
            self.arrayOfInts.push(self.stream.read_u1()?);
        }
        self.testType = Box::new(DebugEnumName__TestSubtype::new(self.stream, self, _root)?);
    }
}

impl DebugEnumName {
}
enum DebugEnumName__TestEnum1 {
    ENUM_VALUE_80,
}
enum DebugEnumName__TestEnum2 {
    ENUM_VALUE_65,
}
#[derive(Default)]
pub struct DebugEnumName__TestSubtype {
    pub field1: Box<DebugEnumName__TestSubtype__InnerEnum1>,
    pub field2: u8,
    pub instanceField: Option<Box<DebugEnumName__TestSubtype__InnerEnum2>>,
}

impl KaitaiStruct for DebugEnumName__TestSubtype {
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
        self.field1 = self.stream.read_u1()?;
        self.field2 = self.stream.read_u1()?;
    }
}

impl DebugEnumName__TestSubtype {
    fn instanceField(&mut self) -> Box<DebugEnumName__TestSubtype__InnerEnum2> {
        if let Some(x) = self.instanceField {
            return x;
        }

        self.instanceField = (self.field2 & 15);
        return self.instanceField;
    }
}
enum DebugEnumName__TestSubtype__InnerEnum1 {
    ENUM_VALUE_67,
}
enum DebugEnumName__TestSubtype__InnerEnum2 {
    ENUM_VALUE_11,
}
