// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ProcessCoerceBytes {
    pub records: Vec<ProcessCoerceBytes_Record>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ProcessCoerceBytes {
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
            // condRepeatExprHeader(NamedIdentifier(records), _io, UserTypeInstream(List(record),None,List()), IntNum(2))
            // handleAssignmentRepeatExpr(NamedIdentifier(records), Self::read_into::<S, ProcessCoerceBytes_Record>(_io, _root, _parent.push(self))?.into())
        }
        Ok(())
    }
}
impl<'r, 's: 'r> ProcessCoerceBytes {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ProcessCoerceBytes::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
// extraAttrForIO(NamedIdentifier(buf_proc), NoRepeat)

#[derive(Default, Debug, PartialEq)]
pub struct ProcessCoerceBytes_Record {
    pub flag: u8,
    pub buf_unproc: Vec<u8>,
    pub buf_proc: Vec<u8>,
    pub raw_buf_proc: Vec<u8>,
    pub buf: Option<Vec<u8>>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ProcessCoerceBytes_Record {
    type Root = ProcessCoerceBytes;
    type ParentStack = (&'r ProcessCoerceBytes, <ProcessCoerceBytes as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.flag = _io.read_u1()?;
        {
            // condIfHeader(Compare(Name(identifier(flag)),Eq,IntNum(0)))
            self.buf_unproc = _io.read_bytes(4 as usize)?.to_vec();
        }
        {
            // condIfHeader(Compare(Name(identifier(flag)),NotEq,IntNum(0)))
            // attrProcess(ProcessXor(IntNum(170)), RawIdentifier(NamedIdentifier(buf_proc)), NamedIdentifier(buf_proc), NoRepeat)
        }
        Ok(())
    }
}
impl<'r, 's: 'r> ProcessCoerceBytes_Record {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ProcessCoerceBytes_Record::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn buf<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r ProcessCoerceBytes>,
        _parent: Option<TypedStack<(&'r ProcessCoerceBytes, <ProcessCoerceBytes as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&Vec<u8>> {
        if self.buf.is_some() {
            return Ok(self.buf.as_ref().unwrap());
        }
        self.buf = Some(if self.flag == 0 { self.buf_unproc } else { self.buf_proc} as Vec<u8>);
        return Ok(self.buf.as_ref().unwrap());
    }
}
