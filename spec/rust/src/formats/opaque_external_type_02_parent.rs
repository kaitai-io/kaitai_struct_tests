// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
use crate::opaque_external_type_02_child::OpaqueExternalType02Child;

#[derive(Default, Debug, PartialEq)]
pub struct OpaqueExternalType02Parent {
    pub parent: Option<OpaqueExternalType02Parent_ParentObj>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for OpaqueExternalType02Parent {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.parent = Some(Self::read_into::<BytesReader, OpaqueExternalType02Parent_ParentObj>(Self::read_into::<S, OpaqueExternalType02Parent_ParentObj>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> OpaqueExternalType02Parent {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = OpaqueExternalType02Parent::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct OpaqueExternalType02Parent_ParentObj {
    pub child: Option<Box<OpaqueExternalType02Child>>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for OpaqueExternalType02Parent_ParentObj {
    type Root = OpaqueExternalType02Parent;
    type ParentStack = (&'r OpaqueExternalType02Parent, <OpaqueExternalType02Parent as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.child = Some(Self::read_into::<BytesReader, Box<OpaqueExternalType02Child>>(Self::read_into::<S, OpaqueExternalType02Child>(_io)?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> OpaqueExternalType02Parent_ParentObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = OpaqueExternalType02Parent_ParentObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
