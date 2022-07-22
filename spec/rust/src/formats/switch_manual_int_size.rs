// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualIntSize {
    pub chunks: Vec<SwitchManualIntSize_Chunk>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualIntSize {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.chunks = Vec::new();
        {
            type ArrayElement = SwitchManualIntSize_Chunk;
            while !_io.is_eof() {
                self.chunks.push(Self::read_into::<S, ArrayElement>(_io, _root, _parent.push(self))?);
            }
        }
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchManualIntSize {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualIntSize::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
// extraAttrForIO(RawIdentifier(NamedIdentifier(body)), NoRepeat)
// extraAttrForIO(RawIdentifier(NamedIdentifier(body)), NoRepeat)

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualIntSize_Chunk {
    pub code: u8,
    pub size: u32,
    pub body: Option<SwitchManualIntSize_Chunk_Body>,
    pub raw_body: Vec<u8>,
}
#[derive(Debug, PartialEq)]
pub enum SwitchManualIntSize_Chunk_Body {
    SwitchManualIntSize_Chunk_ChunkMeta(SwitchManualIntSize_Chunk_ChunkMeta),
    SwitchManualIntSize_Chunk_ChunkDir(SwitchManualIntSize_Chunk_ChunkDir),
    Bytes(Vec<u8>),
}
impl From<SwitchManualIntSize_Chunk_ChunkMeta> for SwitchManualIntSize_Chunk_Body {
    fn from(v: SwitchManualIntSize_Chunk_ChunkMeta) -> Self {
        Self::SwitchManualIntSize_Chunk_ChunkMeta(v)
    }
}
impl From<SwitchManualIntSize_Chunk_ChunkDir> for SwitchManualIntSize_Chunk_Body {
    fn from(v: SwitchManualIntSize_Chunk_ChunkDir) -> Self {
        Self::SwitchManualIntSize_Chunk_ChunkDir(v)
    }
}
impl From<Vec<u8>> for SwitchManualIntSize_Chunk_Body {
    fn from(v: Vec<u8>) -> Self {
        Self::Bytes(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualIntSize_Chunk {
    type Root = SwitchManualIntSize;
    type ParentStack = (&'r SwitchManualIntSize, <SwitchManualIntSize as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.code = _io.read_u1()?;
        self.size = _io.read_u4le()?;
        match self.code {
            17 => {
                self.body = Some(&BytesReader::new(_io.read_bytes(self.size as usize)?));
            }
            34 => {
                self.body = Some(&BytesReader::new(_io.read_bytes(self.size as usize)?));
            }
            // switchElseStart()
            self.body = Some(_io.read_bytes(self.size as usize)?);
        }
        _ => panic!("unhandled value")
    }
    Ok(())
}
}
impl<'r, 's: 'r> SwitchManualIntSize_Chunk {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualIntSize_Chunk::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualIntSize_Chunk_ChunkMeta {
pub title: String,
pub author: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualIntSize_Chunk_ChunkMeta {
type Root = SwitchManualIntSize;
type ParentStack = (&'r SwitchManualIntSize_Chunk, <SwitchManualIntSize_Chunk as KStruct<'r, 's>>::ParentStack);

fn read<S: KStream>(
    &mut self,
    _io: &'s S,
    _root: Option<&'r Self::Root>,
    _parent: Option<TypedStack<Self::ParentStack>>
) -> KResult<()> {
    self.title = decode_string(_io.read_bytes_term(0, false, true, true)?, "UTF-8")?;
    self.author = decode_string(_io.read_bytes_term(0, false, true, true)?, "UTF-8")?;
    Ok(())
}
}
impl<'r, 's: 'r> SwitchManualIntSize_Chunk_ChunkMeta {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualIntSize_Chunk_ChunkMeta::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualIntSize_Chunk_ChunkDir {
pub entries: Vec<String>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualIntSize_Chunk_ChunkDir {
type Root = SwitchManualIntSize;
type ParentStack = (&'r SwitchManualIntSize_Chunk, <SwitchManualIntSize_Chunk as KStruct<'r, 's>>::ParentStack);

fn read<S: KStream>(
    &mut self,
    _io: &'s S,
    _root: Option<&'r Self::Root>,
    _parent: Option<TypedStack<Self::ParentStack>>
) -> KResult<()> {
    self.entries = Vec::new();
    {
        type ArrayElement = String;
        while !_io.is_eof() {
            self.entries.push(Self::read_into::<S, ArrayElement>(_io, _root, _parent.push(self))?);
        }
    }
    Ok(())
}
}
impl<'r, 's: 'r> SwitchManualIntSize_Chunk_ChunkDir {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualIntSize_Chunk_ChunkDir::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
