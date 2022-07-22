// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct OpaqueExternalType02Child {
    pub s1: String,
    pub s2: String,
    pub s3: Option<OpaqueExternalType02Child_OpaqueExternalType02ChildChild>,
    pub some_method: Option<bool>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for OpaqueExternalType02Child {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.s1 = decode_string(_io.read_bytes_term(124, false, true, true)?, "UTF-8")?;
        self.s2 = decode_string(_io.read_bytes_term(124, false, false, true)?, "UTF-8")?;
        self.s3 = Some(Self::read_into::<BytesReader, OpaqueExternalType02Child_OpaqueExternalType02ChildChild>(Self::read_into::<S, OpaqueExternalType02Child_OpaqueExternalType02ChildChild>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> OpaqueExternalType02Child {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = OpaqueExternalType02Child::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn some_method<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&bool> {
        if self.some_method.is_some() {
            return Ok(self.some_method.as_ref().unwrap());
        }
        self.some_method = Some(true as bool);
        return Ok(self.some_method.as_ref().unwrap());
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct OpaqueExternalType02Child_OpaqueExternalType02ChildChild {
    pub s3: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for OpaqueExternalType02Child_OpaqueExternalType02ChildChild {
    type Root = OpaqueExternalType02Child;
    type ParentStack = (&'r OpaqueExternalType02Child, <OpaqueExternalType02Child as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        {
            // condIfHeader(Attribute(Name(identifier(_root)),identifier(some_method)))
            self.s3 = decode_string(_io.read_bytes_term(64, true, true, true)?, "UTF-8")?;
        }
        Ok(())
    }
}
impl<'r, 's: 'r> OpaqueExternalType02Child_OpaqueExternalType02ChildChild {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = OpaqueExternalType02Child_OpaqueExternalType02ChildChild::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
