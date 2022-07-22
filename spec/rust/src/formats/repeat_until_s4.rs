// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct RepeatUntilS4 {
    pub entries: Vec<i32>,
    pub afterall: String,
}
impl<'r, 's: 'r> KStruct<'r, 's> for RepeatUntilS4 {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.entries = Vec::new();
        {
            // condRepeatUntilHeader(NamedIdentifier(entries), _io, IntMultiType(true,Width4,Some(LittleEndian)), Compare(Name(identifier(_)),Eq,UnaryOp(Minus,IntNum(1))))
            // handleAssignmentRepeatUntil(NamedIdentifier(entries), _io.read_s4le()?, false)
            // condRepeatUntilFooter(NamedIdentifier(entries), _io, IntMultiType(true,Width4,Some(LittleEndian)), Compare(Name(identifier(_)),Eq,UnaryOp(Minus,IntNum(1))))
        } {}
        self.afterall = decode_string(_io.read_bytes_term(0, false, true, true)?, "ASCII")?;
        Ok(())
    }
}
impl<'r, 's: 'r> RepeatUntilS4 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = RepeatUntilS4::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
