// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
// extraAttrForIO(NamedIdentifier(bufs), RepeatExpr(IntNum(2)))

#[derive(Default, Debug, PartialEq)]
pub struct ProcessRepeatBytes {
    pub bufs: Vec<Vec<u8>>,
    pub raw_bufs: Vec<Vec<u8>>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ProcessRepeatBytes {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.bufs = Vec::new();
        {
            // condRepeatExprHeader(NamedIdentifier(bufs), _io, BytesLimitType(IntNum(5),None,false,None,Some(ProcessXor(IntNum(158)))), IntNum(2))
            // handleAssignmentRepeatExpr(RawIdentifier(NamedIdentifier(bufs)), _io.read_bytes(5 as usize)?)
            // attrProcess(ProcessXor(IntNum(158)), RawIdentifier(NamedIdentifier(bufs)), NamedIdentifier(bufs), RepeatExpr(IntNum(2)))
        }
        Ok(())
    }
}
impl<'r, 's: 'r> ProcessRepeatBytes {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ProcessRepeatBytes::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
