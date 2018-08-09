// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::io::Cursor;
use std::vec::Vec;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;
use opaque_external_type_02_child::OpaqueExternalType02Child;

#[derive(Default)]
pub struct OpaqueExternalType02Parent {
    pub parent: Box<OpaqueExternalType02Parent__ParentObj>,
}

impl KaitaiStruct for OpaqueExternalType02Parent {
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
        self.parent = Box::new(OpaqueExternalType02Parent__ParentObj::new(self.stream, self, _root)?);
    }
}

impl OpaqueExternalType02Parent {
}
#[derive(Default)]
pub struct OpaqueExternalType02Parent__ParentObj {
    pub child: Box<OpaqueExternalType02Child>,
}

impl KaitaiStruct for OpaqueExternalType02Parent__ParentObj {
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
        self.child = Box::new(OpaqueExternalType02Child::new(self.stream, self, _root)?);
    }
}

impl OpaqueExternalType02Parent__ParentObj {
}
