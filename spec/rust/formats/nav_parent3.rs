// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct NavParent3 {
    pub ofs_tags: u32,
    pub num_tags: u32,
    pub tags: Vec<NavParent3_Tag>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavParent3 {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.ofs_tags = _io.read_u4le()?;
        self.num_tags = _io.read_u4le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> NavParent3 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParent3::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn tags<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&Vec<NavParent3_Tag>> {
        if self.tags.is_some() {
            return Ok(self.tags.as_ref().unwrap());
        }
        // pushPos(_io)
        // seek(_io, Name(identifier(ofs_tags)))
        self.tags = Vec::new();
        {
            // condRepeatExprHeader(InstanceIdentifier(tags), _io, UserTypeInstream(List(tag),None,List()), Name(identifier(num_tags)))
            // handleAssignmentRepeatExpr(InstanceIdentifier(tags), Self::read_into::<S, NavParent3_Tag>(_io, _root, _parent.push(self))?.into())
        }
        // popPos(_io)
        return Ok(self.tags.as_ref().unwrap());
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct NavParent3_Tag {
    pub name: String,
    pub ofs: u32,
    pub num_items: u32,
    pub tag_content: Option<NavParent3_Tag_TagContent>,
}
#[derive(Debug, PartialEq)]
pub enum NavParent3_Tag_TagContent {
    NavParent3_Tag_TagChar(NavParent3_Tag_TagChar),
}
impl From<NavParent3_Tag_TagChar> for NavParent3_Tag_TagContent {
    fn from(v: NavParent3_Tag_TagChar) -> Self {
        Self::NavParent3_Tag_TagChar(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for NavParent3_Tag {
    type Root = NavParent3;
    type ParentStack = (&'r NavParent3, <NavParent3 as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.name = decode_string(_io.read_bytes(4 as usize)?, "ASCII")?;
        self.ofs = _io.read_u4le()?;
        self.num_items = _io.read_u4le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> NavParent3_Tag {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParent3_Tag::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn tag_content<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r NavParent3>,
        _parent: Option<TypedStack<(&'r NavParent3, <NavParent3 as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&NavParent3_Tag_TagContent> {
        if self.tag_content.is_some() {
            return Ok(self.tag_content.as_ref().unwrap());
        }
        // pushPos(// useIO(Attribute(Name(identifier(_root)),identifier(_io))))
        // seek(// useIO(Attribute(Name(identifier(_root)),identifier(_io))), Name(identifier(ofs)))
        {
            let on = &self.name;
            if on == "RAHC" {
            }
        }
        // popPos(// useIO(Attribute(Name(identifier(_root)),identifier(_io))))
        return Ok(self.tag_content.as_ref().unwrap());
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct NavParent3_Tag_TagChar {
    pub content: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NavParent3_Tag_TagChar {
    type Root = NavParent3;
    type ParentStack = (&'r NavParent3_Tag, <NavParent3_Tag as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.content = decode_string(_io.read_bytes(_parent.peek().num_items as usize)?, "ASCII")?;
        Ok(())
    }
}
impl<'r, 's: 'r> NavParent3_Tag_TagChar {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NavParent3_Tag_TagChar::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
