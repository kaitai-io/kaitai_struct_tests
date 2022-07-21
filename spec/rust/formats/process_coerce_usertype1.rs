// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ProcessCoerceUsertype1 {
    pub records: Vec<ProcessCoerceUsertype1_Record>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ProcessCoerceUsertype1 {
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
            // handleAssignmentRepeatExpr(NamedIdentifier(records), Self::read_into::<S, ProcessCoerceUsertype1_Record>(_io, _root, _parent.push(self))?.into())
        }
        Ok(())
    }
}
impl<'r, 's: 'r> ProcessCoerceUsertype1 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ProcessCoerceUsertype1::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
// extraAttrForIO(RawIdentifier(NamedIdentifier(buf_unproc)), NoRepeat)
// extraAttrForIO(RawIdentifier(NamedIdentifier(buf_proc)), NoRepeat)
// extraAttrForIO(RawIdentifier(NamedIdentifier(buf_proc)), NoRepeat)

#[derive(Default, Debug, PartialEq)]
pub struct ProcessCoerceUsertype1_Record {
    pub flag: u8,
    pub buf_unproc: Option<ProcessCoerceUsertype1_Foo>,
    pub buf_proc: Option<ProcessCoerceUsertype1_Foo>,
    pub raw_buf_unproc: Vec<u8>,
    pub raw_buf_proc: Vec<u8>,
    pub raw_raw_buf_proc: Vec<u8>,
    pub buf: Option<ProcessCoerceUsertype1_Foo>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ProcessCoerceUsertype1_Record {
    type Root = ProcessCoerceUsertype1;
    type ParentStack = (&'r ProcessCoerceUsertype1, <ProcessCoerceUsertype1 as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.flag = _io.read_u1()?;
        {
            // condIfHeader(Compare(Name(identifier(flag)),Eq,IntNum(0)))
            self.buf_unproc = Some(Self::read_into::<BytesReader, ProcessCoerceUsertype1_Foo>(&BytesReader::new(_io.read_bytes(4 as usize)?), Some(self), _parent.push(self))?);
        }
        {
            // condIfHeader(Compare(Name(identifier(flag)),NotEq,IntNum(0)))
            // attrProcess(ProcessXor(IntNum(170)), RawIdentifier(RawIdentifier(NamedIdentifier(buf_proc))), RawIdentifier(NamedIdentifier(buf_proc)), NoRepeat)
            self.buf_proc = Some(Self::read_into::<BytesReader, ProcessCoerceUsertype1_Foo>(&BytesReader::new(_io.read_bytes(4 as usize)?), Some(self), _parent.push(self))?);
        }
        Ok(())
    }
}
impl<'r, 's: 'r> ProcessCoerceUsertype1_Record {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ProcessCoerceUsertype1_Record::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn buf<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r ProcessCoerceUsertype1>,
        _parent: Option<TypedStack<(&'r ProcessCoerceUsertype1, <ProcessCoerceUsertype1 as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&ProcessCoerceUsertype1_Foo> {
        if self.buf.is_some() {
            return Ok(self.buf.as_ref().unwrap());
        }
        self.buf = Some(if self.flag == 0 { self.buf_unproc } else { self.buf_proc} as Foo);
        return Ok(self.buf.as_ref().unwrap());
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct ProcessCoerceUsertype1_Foo {
    pub value: u32,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ProcessCoerceUsertype1_Foo {
    type Root = ProcessCoerceUsertype1;
    type ParentStack = (&'r ProcessCoerceUsertype1, <ProcessCoerceUsertype1 as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.value = _io.read_u4le()?;
        Ok(())
    }
}
impl<'r, 's: 'r> ProcessCoerceUsertype1_Foo {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ProcessCoerceUsertype1_Foo::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
