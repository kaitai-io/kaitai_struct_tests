// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct NavParent {
    pub header: Option<NavParent_HeaderObj>,
    pub index: Option<NavParent_IndexObj>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavParent {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.header = Some(Self::read_into::<BytesReader, NavParent_HeaderObj>(Self::read_into::<S, NavParent_HeaderObj>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.index = Some(Self::read_into::<BytesReader, NavParent_IndexObj>(Self::read_into::<S, NavParent_IndexObj>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> NavParent {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParent::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NavParent_HeaderObj {
    pub qty_entries: u32,
    pub filename_len: u32,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavParent_HeaderObj {
    type Root = NavParent;
    type ParentStack = (&'r NavParent, <NavParent as KStruct<'r, 's>>::ParentStack);

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
impl<'r, 's: 'r> NavParent_HeaderObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParent_HeaderObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NavParent_IndexObj {
    pub magic: Vec<u8>,
    pub entries: Vec<NavParent_Entry>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavParent_IndexObj {
    type Root = NavParent;
    type ParentStack = (&'r NavParent, <NavParent as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.magic = _io.read_bytes(4 as usize)?.to_vec();
        self.entries = Vec::new();
        {
            // condRepeatExprHeader(NamedIdentifier(entries), _io, UserTypeInstream(List(entry),None,List()), Attribute(Attribute(Name(identifier(_parent)),identifier(header)),identifier(qty_entries)))
            // handleAssignmentRepeatExpr(NamedIdentifier(entries), Self::read_into::<S, NavParent_Entry>(_io, _root, _parent.push(self))?.into())
        }
        Ok(())
    }
}
impl<'r, 's: 'r> NavParent_IndexObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParent_IndexObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NavParent_Entry {
    pub filename: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavParent_Entry {
    type Root = NavParent;
    type ParentStack = (&'r NavParent, <NavParent as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.filename = decode_string(_io.read_bytes(_parent.peek()._parent.header.filename_len as usize)?, "UTF-8")?;
        Ok(())
    }
}
impl<'r, 's: 'r> NavParent_Entry {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParent_Entry::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
