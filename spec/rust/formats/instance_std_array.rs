// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct InstanceStdArray {
    pub ofs: u32,
    pub entry_size: u32,
    pub qty_entries: u32,
    pub entries: Vec<Vec<u8>>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for InstanceStdArray {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.ofs = _io.read_u4le()?;
        self.entry_size = _io.read_u4le()?;
        self.qty_entries = _io.read_u4le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> InstanceStdArray {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = InstanceStdArray::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn entries<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&Vec<Vec<u8>>> {
        if self.entries.is_some() {
            return Ok(self.entries.as_ref().unwrap());
        }
        // pushPos(_io)
        // seek(_io, Name(identifier(ofs)))
        self.entries = Vec::new();
        {
            // condRepeatExprHeader(InstanceIdentifier(entries), _io, BytesLimitType(Name(identifier(entry_size)),None,false,None,None), Name(identifier(qty_entries)))
            // handleAssignmentRepeatExpr(InstanceIdentifier(entries), _io.read_bytes(self.entry_size as usize)?)
        }
        // popPos(_io)
        return Ok(self.entries.as_ref().unwrap());
    }
}
