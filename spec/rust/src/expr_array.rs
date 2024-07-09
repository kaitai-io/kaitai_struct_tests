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
pub struct ExprArray {
    pub aint: Vec<u32>,
    pub afloat: Vec<f64>,
    pub astr: Vec<String>,
    pub aintFirst: Option<u32>,
    pub afloatSize: Option<i32>,
    pub astrSize: Option<i32>,
    pub aintMin: Option<u32>,
    pub afloatMin: Option<f64>,
    pub aintSize: Option<i32>,
    pub aintLast: Option<u32>,
    pub afloatLast: Option<f64>,
    pub astrFirst: Option<String>,
    pub astrLast: Option<String>,
    pub aintMax: Option<u32>,
    pub afloatFirst: Option<f64>,
    pub astrMin: Option<String>,
    pub astrMax: Option<String>,
    pub afloatMax: Option<f64>,
}

impl KaitaiStruct for ExprArray {
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
        self.aint = vec!();
        for i in 0..4 {
            self.aint.push(self.stream.read_u4le()?);
        }
        self.afloat = vec!();
        for i in 0..3 {
            self.afloat.push(self.stream.read_f8le()?);
        }
        self.astr = vec!();
        for i in 0..3 {
            self.astr.push(panic!("Unimplemented encoding for bytesToStr: {}", "UTF-8"));
        }
    }
}

impl ExprArray {
    fn aintFirst(&mut self) -> u32 {
        if let Some(x) = self.aintFirst {
            return x;
        }

        self.aintFirst = self.aint.first();
        return self.aintFirst;
    }
    fn afloatSize(&mut self) -> i32 {
        if let Some(x) = self.afloatSize {
            return x;
        }

        self.afloatSize = self.afloat.len();
        return self.afloatSize;
    }
    fn astrSize(&mut self) -> i32 {
        if let Some(x) = self.astrSize {
            return x;
        }

        self.astrSize = self.astr.len();
        return self.astrSize;
    }
    fn aintMin(&mut self) -> u32 {
        if let Some(x) = self.aintMin {
            return x;
        }

        self.aintMin = self.aint.iter().min();
        return self.aintMin;
    }
    fn afloatMin(&mut self) -> f64 {
        if let Some(x) = self.afloatMin {
            return x;
        }

        self.afloatMin = self.afloat.iter().min();
        return self.afloatMin;
    }
    fn aintSize(&mut self) -> i32 {
        if let Some(x) = self.aintSize {
            return x;
        }

        self.aintSize = self.aint.len();
        return self.aintSize;
    }
    fn aintLast(&mut self) -> u32 {
        if let Some(x) = self.aintLast {
            return x;
        }

        self.aintLast = self.aint.last();
        return self.aintLast;
    }
    fn afloatLast(&mut self) -> f64 {
        if let Some(x) = self.afloatLast {
            return x;
        }

        self.afloatLast = self.afloat.last();
        return self.afloatLast;
    }
    fn astrFirst(&mut self) -> String {
        if let Some(x) = self.astrFirst {
            return x;
        }

        self.astrFirst = self.astr.first();
        return self.astrFirst;
    }
    fn astrLast(&mut self) -> String {
        if let Some(x) = self.astrLast {
            return x;
        }

        self.astrLast = self.astr.last();
        return self.astrLast;
    }
    fn aintMax(&mut self) -> u32 {
        if let Some(x) = self.aintMax {
            return x;
        }

        self.aintMax = self.aint.iter().max();
        return self.aintMax;
    }
    fn afloatFirst(&mut self) -> f64 {
        if let Some(x) = self.afloatFirst {
            return x;
        }

        self.afloatFirst = self.afloat.first();
        return self.afloatFirst;
    }
    fn astrMin(&mut self) -> String {
        if let Some(x) = self.astrMin {
            return x;
        }

        self.astrMin = self.astr.iter().min();
        return self.astrMin;
    }
    fn astrMax(&mut self) -> String {
        if let Some(x) = self.astrMax {
            return x;
        }

        self.astrMax = self.astr.iter().max();
        return self.astrMax;
    }
    fn afloatMax(&mut self) -> f64 {
        if let Some(x) = self.afloatMax {
            return x;
        }

        self.afloatMax = self.afloat.iter().max();
        return self.afloatMax;
    }
}
