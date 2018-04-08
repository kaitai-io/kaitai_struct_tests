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

pub struct ExprArray {
    pub aintFirst: u32,
    pub afloatSize: i32,
    pub astrSize: i32,
    pub aintMin: u32,
    pub afloatMin: f64,
    pub aintSize: i32,
    pub aintLast: u32,
    pub afloatLast: f64,
    pub astrFirst: String,
    pub astrLast: String,
    pub aintMax: u32,
    pub afloatFirst: f64,
    pub astrMin: String,
    pub astrMax: String,
    pub afloatMax: f64,
    pub aint: Vec<u32>*,
    pub afloat: Vec<f64>*,
    pub astr: Vec<String>*,
}

impl KaitaiStruct for ExprArray {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            aintFirst: 0,
            afloatSize: i32,
            astrSize: i32,
            aintMin: 0,
            afloatMin: f64,
            aintSize: i32,
            aintLast: 0,
            afloatLast: f64,
            astrFirst: String,
            astrLast: String,
            aintMax: 0,
            afloatFirst: f64,
            astrMin: String,
            astrMax: String,
            afloatMax: f64,
            aint: Vec<u32>*,
            afloat: Vec<f64>*,
            astr: Vec<String>*,
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
        self.aint = [];
        $n = 4;
        for ($i = 0; $i < $n; $i++) {
            self.aint[] = stream.read_u4le()?;
        }
        self.afloat = [];
        $n = 3;
        for ($i = 0; $i < $n; $i++) {
            self.afloat[] = stream.read_f8le()?;
        }
        self.astr = [];
        $n = 3;
        for ($i = 0; $i < $n; $i++) {
            self.astr[] = &mut S::bytesToStr(stream->readBytesTerm(0, false, true, true), "UTF-8");
        }

        Ok(())
    }
    public function aintFirst() {
        if (self.aintFirst !== null)
            return self.aintFirst;
        self.aintFirst = $this->aint()[0];
        return self.aintFirst;
    }
    public function afloatSize() {
        if (self.afloatSize !== null)
            return self.afloatSize;
        self.afloatSize = count($this->afloat());
        return self.afloatSize;
    }
    public function astrSize() {
        if (self.astrSize !== null)
            return self.astrSize;
        self.astrSize = count($this->astr());
        return self.astrSize;
    }
    public function aintMin() {
        if (self.aintMin !== null)
            return self.aintMin;
        self.aintMin = min($this->aint());
        return self.aintMin;
    }
    public function afloatMin() {
        if (self.afloatMin !== null)
            return self.afloatMin;
        self.afloatMin = min($this->afloat());
        return self.afloatMin;
    }
    public function aintSize() {
        if (self.aintSize !== null)
            return self.aintSize;
        self.aintSize = count($this->aint());
        return self.aintSize;
    }
    public function aintLast() {
        if (self.aintLast !== null)
            return self.aintLast;
        self.aintLast = $this->aint()[count($this->aint()) - 1];
        return self.aintLast;
    }
    public function afloatLast() {
        if (self.afloatLast !== null)
            return self.afloatLast;
        self.afloatLast = $this->afloat()[count($this->afloat()) - 1];
        return self.afloatLast;
    }
    public function astrFirst() {
        if (self.astrFirst !== null)
            return self.astrFirst;
        self.astrFirst = $this->astr()[0];
        return self.astrFirst;
    }
    public function astrLast() {
        if (self.astrLast !== null)
            return self.astrLast;
        self.astrLast = $this->astr()[count($this->astr()) - 1];
        return self.astrLast;
    }
    public function aintMax() {
        if (self.aintMax !== null)
            return self.aintMax;
        self.aintMax = max($this->aint());
        return self.aintMax;
    }
    public function afloatFirst() {
        if (self.afloatFirst !== null)
            return self.afloatFirst;
        self.afloatFirst = $this->afloat()[0];
        return self.afloatFirst;
    }
    public function astrMin() {
        if (self.astrMin !== null)
            return self.astrMin;
        self.astrMin = min($this->astr());
        return self.astrMin;
    }
    public function astrMax() {
        if (self.astrMax !== null)
            return self.astrMax;
        self.astrMax = max($this->astr());
        return self.astrMax;
    }
    public function afloatMax() {
        if (self.afloatMax !== null)
            return self.afloatMax;
        self.afloatMax = max($this->afloat());
        return self.afloatMax;
    }
}
