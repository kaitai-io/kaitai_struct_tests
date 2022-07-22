// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct NonStandard {
    pub foo: u8,
    pub bar: Option<NonStandard_Bar>,
    pub vi: Option<u8>,
    pub pi: Option<u8>,
}
#[derive(Debug, PartialEq)]
pub enum NonStandard_Bar {
    U2(u16),
    U4(u32),
}
impl From<u16> for NonStandard_Bar {
    fn from(v: u16) -> Self {
        Self::U2(v)
    }
}
impl From<u32> for NonStandard_Bar {
    fn from(v: u32) -> Self {
        Self::U4(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for NonStandard {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.foo = _io.read_u1()?;
        match self.foo {
            42 => {
                self.bar = Some(_io.read_u2le()?);
            }
            43 => {
                self.bar = Some(_io.read_u4le()?);
            }
            _ => panic!("unhandled value")
        }
        Ok(())
    }
}
impl<'r, 's: 'r> NonStandard {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NonStandard::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn vi<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u8> {
        if self.vi.is_some() {
            return Ok(self.vi.as_ref().unwrap());
        }
        self.vi = Some(self.foo as u8);
        return Ok(self.vi.as_ref().unwrap());
    }
    fn pi<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u8> {
        if self.pi.is_some() {
            return Ok(self.pi.as_ref().unwrap());
        }
        // pushPos(_io)
        // seek(_io, IntNum(0))
        // popPos(_io)
        return Ok(self.pi.as_ref().unwrap());
    }
}
