// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct BitsSeqEndianCombo {
    pub be1: u64,
    pub be2: u64,
    pub le3: u64,
    pub be4: u64,
    pub le5: u64,
    pub le6: u64,
    pub le7: u64,
    pub be8: bool,
}
impl<'r, 's: 'r> KStruct<'r, 's> for BitsSeqEndianCombo {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.be1 = _io.read_bits_int(6)?;
        self.be2 = _io.read_bits_int(10)?;
        self.le3 = _io.read_bits_int(8)?;
        self.be4 = _io.read_bits_int(8)?;
        self.le5 = _io.read_bits_int(5)?;
        self.le6 = _io.read_bits_int(6)?;
        self.le7 = _io.read_bits_int(5)?;
        self.be8 = _io.read_bits_int(1)? != 0;
        Ok(())
    }
}
impl<'r, 's: 'r> BitsSeqEndianCombo {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = BitsSeqEndianCombo::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
