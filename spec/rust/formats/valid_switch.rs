// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ValidSwitch {
    pub a: u8,
    pub b: Option<ValidSwitch_B>,
}
#[derive(Debug, PartialEq)]
pub enum ValidSwitch_B {
    U2(u16),
    U2(u16),
}
impl From<u16> for ValidSwitch_B {
    fn from(v: u16) -> Self {
        Self::U2(v)
    }
}
impl From<u16> for ValidSwitch_B {
    fn from(v: u16) -> Self {
        Self::U2(v)
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for ValidSwitch {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.a = _io.read_u1()?;
        match self.a {
            80 => {
                self.b = Some(_io.read_u2le()?);
            }
            // switchElseStart()
            self.b = Some(_io.read_u2be()?);
        }
        _ => panic!("unhandled value")
    }
    Ok(())
}
}
impl<'r, 's: 'r> ValidSwitch {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ValidSwitch::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
