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

pub struct NavParent2 {
pub struct Tag {
pub struct TagChar {
    pub ofsTags: u32,
    pub numTags: u32,
    pub tags: Vec<>*,
    pub tagContent: ,
    pub name: String,
    pub ofs: u32,
    pub numItems: u32,
    pub content: String,
}

impl KaitaiStruct for NavParent2 {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for Tag {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
                }

                impl KaitaiStruct for TagChar {
                    fn new<S: KaitaiStream>(stream: &mut S,
                                            _parent: &Option<Box<KaitaiStruct>>,
                                            _root: &Option<Box<KaitaiStruct>>)
                                            -> Result<Self>
                        where Self: Sized {
                        let mut s = Self {
            ofsTags: 0,
            numTags: 0,
            tags: Vec<>*,
            tagContent: ,
            name: String,
            ofs: 0,
            numItems: 0,
            content: String,
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
        self.ofsTags = stream.read_u4le()?;
        self.numTags = stream.read_u4le()?;
        self.tags = [];
        $n = $this->numTags();
        for ($i = 0; $i < $n; $i++) {
            self.tags[] = new nav_parent2::tag(stream, $this, _root);
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
        self.name = &mut S::bytesToStr(stream->readBytes(4), "ASCII");
        self.ofs = stream.read_u4le()?;
        self.numItems = stream.read_u4le()?;

        Ok(())
    }
    public function tagContent() {
        if (self.tagContent !== null)
            return self.tagContent;
        $io = $this->_root()->_io();
        $_pos = $io->pos();
        $io->seek($this->ofs());
        switch ($this->name()) {
            case "RAHC":
                self.tagContent = new nav_parent2::tag::tag_char($io, $this, _root);
                break;
        }
        $io->seek($_pos);
        return self.tagContent;
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
        self.content = &mut S::bytesToStr(stream->readBytes($this->_parent()->numItems()), "ASCII");

        Ok(())
    }
}
