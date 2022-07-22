// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualIntSizeElse {
    pub chunks: Vec<SwitchManualIntSizeElse_Chunk>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualIntSizeElse {
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
            type ArrayElement = SwitchManualIntSizeElse_Chunk;
            while !_io.is_eof() {
                self.chunks.push(Self::read_into::<S, ArrayElement>(_io, _root, _parent.push(self))?);
            }
        }
        Ok(())
    }
}
impl<'r, 's: 'r> SwitchManualIntSizeElse {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualIntSizeElse::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
// extraAttrForIO(RawIdentifier(NamedIdentifier(body)), NoRepeat)
// extraAttrForIO(RawIdentifier(NamedIdentifier(body)), NoRepeat)
// extraAttrForIO(RawIdentifier(NamedIdentifier(body)), NoRepeat)

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualIntSizeElse_Chunk {
    pub code: u8,
    pub size: u32,
    pub body: Option<SwitchManualIntSizeElse_Chunk_Body>,
    pub raw_body: Vec<u8>,
}
#[derive(Debug, PartialEq)]
pub enum SwitchManualIntSizeElse_Chunk_Body {
    SwitchManualIntSizeElse_Chunk_ChunkMeta(SwitchManualIntSizeElse_Chunk_ChunkMeta),
    SwitchManualIntSizeElse_Chunk_ChunkDir(SwitchManualIntSizeElse_Chunk_ChunkDir),
    SwitchManualIntSizeElse_Chunk_Dummy(SwitchManualIntSizeElse_Chunk_Dummy),
}
impl From<SwitchManualIntSizeElse_Chunk_ChunkMeta> for SwitchManualIntSizeElse_Chunk_Body {
    fn from(v: SwitchManualIntSizeElse_Chunk_ChunkMeta) -> Self {
        Self::SwitchManualIntSizeElse_Chunk_ChunkMeta(v)
    }
}
impl From<SwitchManualIntSizeElse_Chunk_ChunkDir> for SwitchManualIntSizeElse_Chunk_Body {
    fn from(v: SwitchManualIntSizeElse_Chunk_ChunkDir) -> Self {
        Self::SwitchManualIntSizeElse_Chunk_ChunkDir(v)
    }
}
impl From<SwitchManualIntSizeElse_Chunk_Dummy> for SwitchManualIntSizeElse_Chunk_Body {
    fn from(v: SwitchManualIntSizeElse_Chunk_Dummy) -> Self {
        Self::SwitchManualIntSizeElse_Chunk_Dummy(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualIntSizeElse_Chunk {
    type Root = SwitchManualIntSizeElse;
    type ParentStack = (&'r SwitchManualIntSizeElse, <SwitchManualIntSizeElse as KStruct<'r, 's>>::ParentStack);

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
            self.body = Some(&BytesReader::new(_io.read_bytes(self.size as usize)?));
        }
        _ => panic!("unhandled value")
    }
    Ok(())
}
}
impl<'r, 's: 'r> SwitchManualIntSizeElse_Chunk {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualIntSizeElse_Chunk::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualIntSizeElse_Chunk_ChunkMeta {
pub title: String,
pub author: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualIntSizeElse_Chunk_ChunkMeta {
type Root = SwitchManualIntSizeElse;
type ParentStack = (&'r SwitchManualIntSizeElse_Chunk, <SwitchManualIntSizeElse_Chunk as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> SwitchManualIntSizeElse_Chunk_ChunkMeta {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualIntSizeElse_Chunk_ChunkMeta::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualIntSizeElse_Chunk_ChunkDir {
pub entries: Vec<String>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualIntSizeElse_Chunk_ChunkDir {
type Root = SwitchManualIntSizeElse;
type ParentStack = (&'r SwitchManualIntSizeElse_Chunk, <SwitchManualIntSizeElse_Chunk as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> SwitchManualIntSizeElse_Chunk_ChunkDir {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualIntSizeElse_Chunk_ChunkDir::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct SwitchManualIntSizeElse_Chunk_Dummy {
pub rest: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for SwitchManualIntSizeElse_Chunk_Dummy {
type Root = SwitchManualIntSizeElse;
type ParentStack = (&'r SwitchManualIntSizeElse_Chunk, <SwitchManualIntSizeElse_Chunk as KStruct<'r, 's>>::ParentStack);

fn read<S: KStream>(
    &mut self,
    _io: &'s S,
    _root: Option<&'r Self::Root>,
    _parent: Option<TypedStack<Self::ParentStack>>
) -> KResult<()> {
    self.rest = _io.read_bytes_full()?;
    Ok(())
}
}
impl<'r, 's: 'r> SwitchManualIntSizeElse_Chunk_Dummy {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = SwitchManualIntSizeElse_Chunk_Dummy::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
