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

pub struct SwitchManualIntSizeElse {
pub struct Chunk {
pub struct ChunkMeta {
pub struct ChunkDir {
pub struct Dummy {
    pub chunks: Vec<>*,
    pub code: u8,
    pub size: u32,
    pub body: ,
    pub _raw_body: String,
    pub title: String,
    pub author: String,
    pub entries: Vec<String>*,
    pub rest: String,
}

impl KaitaiStruct for SwitchManualIntSizeElse {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for Chunk {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
                }

                impl KaitaiStruct for ChunkMeta {
                    fn new<S: KaitaiStream>(stream: &mut S,
                                            _parent: &Option<Box<KaitaiStruct>>,
                                            _root: &Option<Box<KaitaiStruct>>)
                                            -> Result<Self>
                        where Self: Sized {
                        let mut s = Self {
                        }

                        impl KaitaiStruct for ChunkDir {
                            fn new<S: KaitaiStream>(stream: &mut S,
                                                    _parent: &Option<Box<KaitaiStruct>>,
                                                    _root: &Option<Box<KaitaiStruct>>)
                                                    -> Result<Self>
                                where Self: Sized {
                                let mut s = Self {
                                }

                                impl KaitaiStruct for Dummy {
                                    fn new<S: KaitaiStream>(stream: &mut S,
                                                            _parent: &Option<Box<KaitaiStruct>>,
                                                            _root: &Option<Box<KaitaiStruct>>)
                                                            -> Result<Self>
                                        where Self: Sized {
                                        let mut s = Self {
            chunks: Vec<>*,
            code: 0,
            size: 0,
            body: ,
            _raw_body: String,
            title: String,
            author: String,
            entries: Vec<String>*,
            rest: String,
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
        self.chunks = [];
        $i = 0;
        while (!stream->isEof()) {
            self.chunks[] = new switch_manual_int_size_else::chunk(stream, $this, _root);
            $i++;
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
        self.code = stream.read_u1()?;
        self.size = stream.read_u4le()?;
        switch ($this->code()) {
            case 17:
                self._raw_body = stream->readBytes($this->size());
                $io = new &mut S(self._raw_body);
                self.body = new switch_manual_int_size_else::chunk::chunk_meta($io, $this, _root);
                break;
            case 34:
                self._raw_body = stream->readBytes($this->size());
                $io = new &mut S(self._raw_body);
                self.body = new switch_manual_int_size_else::chunk::chunk_dir($io, $this, _root);
                break;
            default:
                self._raw_body = stream->readBytes($this->size());
                $io = new &mut S(self._raw_body);
                self.body = new switch_manual_int_size_else::chunk::dummy($io, $this, _root);
                break;
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
        self.title = &mut S::bytesToStr(stream->readBytesTerm(0, false, true, true), "UTF-8");
        self.author = &mut S::bytesToStr(stream->readBytesTerm(0, false, true, true), "UTF-8");

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
        self.entries = [];
        $i = 0;
        while (!stream->isEof()) {
            self.entries[] = &mut S::bytesToStr(stream->readBytes(4), "UTF-8");
            $i++;
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
        self.rest = stream->readBytesFull();

        Ok(())
    }
}
