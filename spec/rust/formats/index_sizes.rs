// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct IndexSizes {
    pub qty: u32,
    pub sizes: Vec<u32>,
    pub bufs: Vec<String>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for IndexSizes {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.qty = _io.read_u4le()?;
        self.sizes = Vec::new();
        {
            // condRepeatExprHeader(NamedIdentifier(sizes), _io, IntMultiType(false,Width4,Some(LittleEndian)), Name(identifier(qty)))
            // handleAssignmentRepeatExpr(NamedIdentifier(sizes), _io.read_u4le()?)
        }
        self.bufs = Vec::new();
        {
            // condRepeatExprHeader(NamedIdentifier(bufs), _io, StrFromBytesType(BytesLimitType(Subscript(Name(identifier(sizes)),Name(identifier(_index))),None,false,None,None),ASCII), Name(identifier(qty)))
            // handleAssignmentRepeatExpr(NamedIdentifier(bufs), decode_string(_io.read_bytes(self.sizes[i as usize] as usize)?, "ASCII")?)
        }
        Ok(())
    }
}
impl<'r, 's: 'r> IndexSizes {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = IndexSizes::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
