// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct RepeatNStrzDouble {
    pub qty: u32,
    pub lines1: Vec<String>,
    pub lines2: Vec<String>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for RepeatNStrzDouble {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.qty = _io.read_u4le()?;
        self.lines1 = Vec::new();
        {
            // condRepeatExprHeader(NamedIdentifier(lines1), _io, StrFromBytesType(BytesTerminatedType(0,false,true,true,None),UTF-8), BinOp(Name(identifier(qty)),Div,IntNum(2)))
            // handleAssignmentRepeatExpr(NamedIdentifier(lines1), decode_string(_io.read_bytes_term(0, false, true, true)?, "UTF-8")?)
        }
        self.lines2 = Vec::new();
        {
            // condRepeatExprHeader(NamedIdentifier(lines2), _io, StrFromBytesType(BytesTerminatedType(0,false,true,true,None),UTF-8), BinOp(Name(identifier(qty)),Div,IntNum(2)))
            // handleAssignmentRepeatExpr(NamedIdentifier(lines2), decode_string(_io.read_bytes_term(0, false, true, true)?, "UTF-8")?)
        }
        Ok(())
    }
}
impl<'r, 's: 'r> RepeatNStrzDouble {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = RepeatNStrzDouble::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
