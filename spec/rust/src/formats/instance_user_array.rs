// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
// extraAttrForIO(RawIdentifier(InstanceIdentifier(user_entries)), RepeatExpr(Name(identifier(qty_entries))))

#[derive(Default, Debug, PartialEq)]
pub struct InstanceUserArray {
    pub ofs: u32,
    pub entry_size: u32,
    pub qty_entries: u32,
    pub raw_user_entries: Vec<Vec<u8>>,
    pub user_entries: Vec<InstanceUserArray_Entry>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for InstanceUserArray {
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
impl<'r, 's: 'r> InstanceUserArray {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = InstanceUserArray::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn user_entries<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&Vec<InstanceUserArray_Entry>> {
        if self.user_entries.is_some() {
            return Ok(self.user_entries.as_ref().unwrap());
        }
        {
            // condIfHeader(Compare(Name(identifier(ofs)),Gt,IntNum(0)))
            // pushPos(_io)
            // seek(_io, Name(identifier(ofs)))
            self.user_entries = Vec::new();
            {
                // condRepeatExprHeader(InstanceIdentifier(user_entries), _io, UserTypeFromBytes(List(entry),None,List(),BytesLimitType(Name(identifier(entry_size)),None,false,None,None),None), Name(identifier(qty_entries)))
                // handleAssignmentRepeatExpr(RawIdentifier(InstanceIdentifier(user_entries)), _io.read_bytes(self.entry_size as usize)?)
                // handleAssignmentRepeatExpr(InstanceIdentifier(user_entries), &BytesReader::new(_io.read_bytes(self.entry_size as usize)?))
            }
            // popPos(_io)
        }
        return Ok(self.user_entries.as_ref().unwrap());
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct InstanceUserArray_Entry {
    pub word1: u16,
    pub word2: u16,
}
impl<'r, 's: 'r> KStruct<'r, 's> for InstanceUserArray_Entry {
    type Root = InstanceUserArray;
    type ParentStack = (&'r InstanceUserArray, <InstanceUserArray as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.word1 = _io.read_u2le()?;
        self.word2 = _io.read_u2le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> InstanceUserArray_Entry {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = InstanceUserArray_Entry::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
