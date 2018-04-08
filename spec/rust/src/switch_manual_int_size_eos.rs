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

pub struct SwitchManualIntSizeEos {
pub struct Chunk {
pub struct ChunkBody {
pub struct ChunkMeta {
pub struct ChunkDir {
    pub chunks: Vec<>*,
    pub code: u8,
    pub size: u32,
    pub body: ,
    pub _raw_body: String,
    pub body: ,
    pub _raw_body: String,
    pub title: String,
    pub author: String,
    pub entries: Vec<String>*,
}

impl KaitaiStruct for SwitchManualIntSizeEos {
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

                impl KaitaiStruct for ChunkBody {
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
            chunks: Vec<>*,
            code: 0,
            size: 0,
            body: ,
            _raw_body: String,
            body: ,
            _raw_body: String,
            title: String,
            author: String,
            entries: Vec<String>*,
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
            self.chunks[] = new switch_manual_int_size_eos::chunk(stream, $this, _root);
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
        self._raw_body = stream->readBytes($this->size());
        $io = new &mut S(self._raw_body);
        self.body = new switch_manual_int_size_eos::chunk_body($io, $this, _root);

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
        switch ($this->_parent()->code()) {
            case 17:
                self._raw_body = stream->readBytesFull();
                $io = new &mut S(self._raw_body);
                self.body = new switch_manual_int_size_eos::chunk_body::chunk_meta($io, $this, _root);
                break;
            case 34:
                self._raw_body = stream->readBytesFull();
                $io = new &mut S(self._raw_body);
                self.body = new switch_manual_int_size_eos::chunk_body::chunk_dir($io, $this, _root);
                break;
            default:
                self.body = stream->readBytesFull();
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
