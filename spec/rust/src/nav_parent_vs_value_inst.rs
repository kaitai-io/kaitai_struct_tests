// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct NavParentVsValueInst {
    pub s1: String,
    pub child: Box<NavParentVsValueInst__ChildObj>,
}

impl KaitaiStruct for NavParentVsValueInst {
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
        self.s1 = panic!("Unimplemented encoding for bytesToStr: {}", "UTF-8");
        self.child = Box::new(NavParentVsValueInst__ChildObj::new(self.stream, self, _root)?);
    }
}

impl NavParentVsValueInst {
}
#[derive(Default)]
pub struct NavParentVsValueInst__ChildObj {
    pub doSomething: Option<bool>,
}

impl KaitaiStruct for NavParentVsValueInst__ChildObj {
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

impl NavParentVsValueInst__ChildObj {
    fn doSomething(&mut self) -> bool {
        if let Some(x) = self.doSomething {
            return x;
        }

        self.doSomething = if self._parent.s1 == "foo" { true } else { false};
        return self.doSomething;
    }
}
