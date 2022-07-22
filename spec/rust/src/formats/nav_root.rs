// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct NavRoot {
    pub header: Option<NavRoot_HeaderObj>,
    pub index: Option<NavRoot_IndexObj>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavRoot {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.header = Some(Self::read_into::<BytesReader, NavRoot_HeaderObj>(Self::read_into::<S, NavRoot_HeaderObj>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.index = Some(Self::read_into::<BytesReader, NavRoot_IndexObj>(Self::read_into::<S, NavRoot_IndexObj>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> NavRoot {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavRoot::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NavRoot_HeaderObj {
    pub qty_entries: u32,
    pub filename_len: u32,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavRoot_HeaderObj {
    type Root = NavRoot;
    type ParentStack = (&'r NavRoot, <NavRoot as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.qty_entries = _io.read_u4le()?;
        self.filename_len = _io.read_u4le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> NavRoot_HeaderObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavRoot_HeaderObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NavRoot_IndexObj {
    pub magic: Vec<u8>,
    pub entries: Vec<NavRoot_Entry>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavRoot_IndexObj {
    type Root = NavRoot;
    type ParentStack = (&'r NavRoot, <NavRoot as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.magic = _io.read_bytes(4 as usize)?.to_vec();
        self.entries = Vec::new();
        {
            // condRepeatExprHeader(NamedIdentifier(entries), _io, UserTypeInstream(List(entry),None,List()), Attribute(Attribute(Name(identifier(_root)),identifier(header)),identifier(qty_entries)))
            // handleAssignmentRepeatExpr(NamedIdentifier(entries), Self::read_into::<S, NavRoot_Entry>(_io, _root, _parent.push(self))?.into())
        }
        Ok(())
    }
}
impl<'r, 's: 'r> NavRoot_IndexObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavRoot_IndexObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NavRoot_Entry {
    pub filename: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavRoot_Entry {
    type Root = NavRoot;
    type ParentStack = (&'r NavRoot, <NavRoot as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.filename = decode_string(_io.read_bytes(_root.ok_or(KError::MissingRoot)?.header.filename_len as usize)?, "UTF-8")?;
        Ok(())
    }
}
impl<'r, 's: 'r> NavRoot_Entry {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavRoot_Entry::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
