// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct RecursiveOne {
    pub one: u8,
    pub next: Option<RecursiveOne_Next>,
}
#[derive(Debug, PartialEq)]
pub enum RecursiveOne_Next {
    RecursiveOne(Box<RecursiveOne>),
    RecursiveOne_Fini(RecursiveOne_Fini),
}
impl From<Box<RecursiveOne>> for RecursiveOne_Next {
    fn from(v: Box<RecursiveOne>) -> Self {
        Self::RecursiveOne(v)
    }
}
impl From<RecursiveOne_Fini> for RecursiveOne_Next {
    fn from(v: RecursiveOne_Fini) -> Self {
        Self::RecursiveOne_Fini(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for RecursiveOne {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.one = _io.read_u1()?;
        match (self.one & 3) {
            0 => {
                self.next = Some(Self::read_into::<S, RecursiveOne>(_io)?.into());
            }
            1 => {
                self.next = Some(Self::read_into::<S, RecursiveOne>(_io)?.into());
            }
            2 => {
                self.next = Some(Self::read_into::<S, RecursiveOne>(_io)?.into());
            }
            3 => {
                self.next = Some(Self::read_into::<S, RecursiveOne_Fini>(_io, _root, _parent.push(self))?.into());
            }
            _ => panic!("unhandled value")
        }
        Ok(())
    }
}
impl<'r, 's: 'r> RecursiveOne {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = RecursiveOne::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct RecursiveOne_Fini {
    pub finisher: u16,
}
impl<'r, 's: 'r> KStruct<'r, 's> for RecursiveOne_Fini {
    type Root = RecursiveOne;
    type ParentStack = (&'r RecursiveOne, <RecursiveOne as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.finisher = _io.read_u2le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> RecursiveOne_Fini {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = RecursiveOne_Fini::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
