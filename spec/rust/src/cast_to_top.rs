// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::io::Cursor;
use std::vec::Vec;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;

#[derive(Default)]
pub struct CastToTop {
    pub code: u8,
    pub header: Option<Box<CastToTop>>,
    pub headerCasted: Option<Box<CastToTop>>,
}

impl KaitaiStruct for CastToTop {
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
        self.code = self.stream.read_u1()?;
    }
}

impl CastToTop {
    fn header(&mut self) -> Box<CastToTop> {
        if let Some(x) = self.header {
            return x;
        }

        let _pos = self.stream.pos();
        self.stream.seek(1);
        self.header = Box::new(CastToTop::new(self.stream, self, _root)?);
        self.stream.seek(_pos);
        return self.header;
    }
    fn headerCasted(&mut self) -> Box<CastToTop> {
        if let Some(x) = self.headerCasted {
            return x;
        }

        self.headerCasted = self.header;
        return self.headerCasted;
    }
}
