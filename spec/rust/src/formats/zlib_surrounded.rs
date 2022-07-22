// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
// extraAttrForIO(RawIdentifier(NamedIdentifier(zlib)), NoRepeat)
// extraAttrForIO(RawIdentifier(NamedIdentifier(zlib)), NoRepeat)

#[derive(Default, Debug, PartialEq)]
pub struct ZlibSurrounded {
    pub pre: Vec<u8>,
    pub zlib: Option<ZlibSurrounded_Inflated>,
    pub post: Vec<u8>,
    pub raw_zlib: Vec<u8>,
    pub raw_raw_zlib: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ZlibSurrounded {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.pre = _io.read_bytes(4 as usize)?.to_vec();
        // attrProcess(ProcessZlib, RawIdentifier(RawIdentifier(NamedIdentifier(zlib))), RawIdentifier(NamedIdentifier(zlib)), NoRepeat)
        self.zlib = Some(Self::read_into::<BytesReader, ZlibSurrounded_Inflated>(&BytesReader::new(_io.read_bytes(12 as usize)?), Some(self), _parent.push(self))?);
        self.post = _io.read_bytes(4 as usize)?.to_vec();
        Ok(())
    }
}
impl<'r, 's: 'r> ZlibSurrounded {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ZlibSurrounded::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct ZlibSurrounded_Inflated {
    pub num: i32,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ZlibSurrounded_Inflated {
    type Root = ZlibSurrounded;
    type ParentStack = (&'r ZlibSurrounded, <ZlibSurrounded as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.num = _io.read_s4le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> ZlibSurrounded_Inflated {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ZlibSurrounded_Inflated::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
