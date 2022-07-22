// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct NestedTypes3 {
    pub a_cc: Option<NestedTypes3_SubtypeA_SubtypeCc>,
    pub a_c_d: Option<NestedTypes3_SubtypeA_SubtypeC_SubtypeD>,
    pub b: Option<NestedTypes3_SubtypeB>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedTypes3 {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.a_cc = Some(Self::read_into::<BytesReader, NestedTypes3_SubtypeA_SubtypeCc>(Self::read_into::<S, NestedTypes3_SubtypeA_SubtypeCc>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.a_c_d = Some(Self::read_into::<BytesReader, NestedTypes3_SubtypeA_SubtypeC_SubtypeD>(Self::read_into::<S, NestedTypes3_SubtypeA_SubtypeC_SubtypeD>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.b = Some(Self::read_into::<BytesReader, NestedTypes3_SubtypeB>(Self::read_into::<S, NestedTypes3_SubtypeB>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> NestedTypes3 {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedTypes3::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NestedTypes3_SubtypeA {
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedTypes3_SubtypeA {
    type Root = NestedTypes3;
    type ParentStack = (&'r NestedTypes3, <NestedTypes3 as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        Ok(())
    }
}
impl<'r, 's: 'r> NestedTypes3_SubtypeA {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedTypes3_SubtypeA::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NestedTypes3_SubtypeA_SubtypeC {
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedTypes3_SubtypeA_SubtypeC {
    type Root = NestedTypes3;
    type ParentStack = (&'r NestedTypes3_SubtypeA, <NestedTypes3_SubtypeA as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        Ok(())
    }
}
impl<'r, 's: 'r> NestedTypes3_SubtypeA_SubtypeC {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedTypes3_SubtypeA_SubtypeC::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NestedTypes3_SubtypeA_SubtypeC_SubtypeD {
    pub value_d: i8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedTypes3_SubtypeA_SubtypeC_SubtypeD {
    type Root = NestedTypes3;
    type ParentStack = (&'r NestedTypes3_SubtypeA_SubtypeC, <NestedTypes3_SubtypeA_SubtypeC as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.value_d = _io.read_s1()?;
        Ok(())
    }
}
impl<'r, 's: 'r> NestedTypes3_SubtypeA_SubtypeC_SubtypeD {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedTypes3_SubtypeA_SubtypeC_SubtypeD::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NestedTypes3_SubtypeA_SubtypeCc {
    pub value_cc: i8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedTypes3_SubtypeA_SubtypeCc {
    type Root = NestedTypes3;
    type ParentStack = (&'r NestedTypes3_SubtypeA, <NestedTypes3_SubtypeA as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.value_cc = _io.read_s1()?;
        Ok(())
    }
}
impl<'r, 's: 'r> NestedTypes3_SubtypeA_SubtypeCc {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedTypes3_SubtypeA_SubtypeCc::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct NestedTypes3_SubtypeB {
    pub value_b: i8,
    pub a_cc: Option<NestedTypes3_SubtypeA_SubtypeCc>,
    pub a_c_d: Option<NestedTypes3_SubtypeA_SubtypeC_SubtypeD>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for NestedTypes3_SubtypeB {
    type Root = NestedTypes3;
    type ParentStack = (&'r NestedTypes3, <NestedTypes3 as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.value_b = _io.read_s1()?;
        self.a_cc = Some(Self::read_into::<BytesReader, NestedTypes3_SubtypeA_SubtypeCc>(Self::read_into::<S, NestedTypes3_SubtypeA_SubtypeCc>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        self.a_c_d = Some(Self::read_into::<BytesReader, NestedTypes3_SubtypeA_SubtypeC_SubtypeD>(Self::read_into::<S, NestedTypes3_SubtypeA_SubtypeC_SubtypeD>(_io, _root, _parent.push(self))?.into(), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> NestedTypes3_SubtypeB {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = NestedTypes3_SubtypeB::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
