// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
// extraAttrForIO(RawIdentifier(NamedIdentifier(records)), RepeatUntil(Compare(Attribute(Name(identifier(_)),identifier(marker)),Eq,IntNum(170))))

#[derive(Default, Debug, PartialEq)]
pub struct RepeatUntilSized {
    pub records: Vec<RepeatUntilSized_Record>,
    pub raw_records: Vec<Vec<u8>>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for RepeatUntilSized {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.records = Vec::new();
        {
            // condRepeatUntilHeader(NamedIdentifier(records), _io, UserTypeFromBytes(List(record),None,List(),BytesLimitType(IntNum(5),None,false,None,None),None), Compare(Attribute(Name(identifier(_)),identifier(marker)),Eq,IntNum(170)))
            // handleAssignmentRepeatUntil(RawIdentifier(NamedIdentifier(records)), _io.read_bytes(5 as usize)?, true)
            // handleAssignmentRepeatUntil(NamedIdentifier(records), &BytesReader::new(_io.read_bytes(5 as usize)?), false)
            // condRepeatUntilFooter(NamedIdentifier(records), _io, UserTypeFromBytes(List(record),None,List(),BytesLimitType(IntNum(5),None,false,None,None),None), Compare(Attribute(Name(identifier(_)),identifier(marker)),Eq,IntNum(170)))
        } {}
        Ok(())
    }
}
impl<'r, 's: 'r> RepeatUntilSized {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = RepeatUntilSized::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct RepeatUntilSized_Record {
    pub marker: u8,
    pub body: u32,
}
impl<'r, 's: 'r> KStruct<'r, 's> for RepeatUntilSized_Record {
    type Root = RepeatUntilSized;
    type ParentStack = (&'r RepeatUntilSized, <RepeatUntilSized as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.marker = _io.read_u1()?;
        self.body = _io.read_u4le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> RepeatUntilSized_Record {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = RepeatUntilSized_Record::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
