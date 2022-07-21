// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct ExprArray {
    pub aint: Vec<u32>,
    pub afloat: Vec<f64>,
    pub astr: Vec<String>,
    pub aint_first: Option<u32>,
    pub afloat_size: Option<i32>,
    pub astr_size: Option<i32>,
    pub aint_min: Option<u32>,
    pub afloat_min: Option<f64>,
    pub aint_size: Option<i32>,
    pub aint_last: Option<u32>,
    pub afloat_last: Option<f64>,
    pub astr_first: Option<String>,
    pub astr_last: Option<String>,
    pub aint_max: Option<u32>,
    pub afloat_first: Option<f64>,
    pub astr_min: Option<String>,
    pub astr_max: Option<String>,
    pub afloat_max: Option<f64>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for ExprArray {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.aint = Vec::new();
        {
            // condRepeatExprHeader(NamedIdentifier(aint), _io, IntMultiType(false,Width4,Some(LittleEndian)), IntNum(4))
            // handleAssignmentRepeatExpr(NamedIdentifier(aint), _io.read_u4le()?)
        }
        self.afloat = Vec::new();
        {
            // condRepeatExprHeader(NamedIdentifier(afloat), _io, FloatMultiType(Width8,Some(LittleEndian)), IntNum(3))
            // handleAssignmentRepeatExpr(NamedIdentifier(afloat), _io.read_f8le()?)
        }
        self.astr = Vec::new();
        {
            // condRepeatExprHeader(NamedIdentifier(astr), _io, StrFromBytesType(BytesTerminatedType(0,false,true,true,None),UTF-8), IntNum(3))
            // handleAssignmentRepeatExpr(NamedIdentifier(astr), decode_string(_io.read_bytes_term(0, false, true, true)?, "UTF-8")?)
        }
        Ok(())
    }
}
impl<'r, 's: 'r> ExprArray {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = ExprArray::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn aint_first<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u32> {
        if self.aint_first.is_some() {
            return Ok(self.aint_first.as_ref().unwrap());
        }
        self.aint_first = Some(self.aint.first() as u32);
        return Ok(self.aint_first.as_ref().unwrap());
    }
    fn afloat_size<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.afloat_size.is_some() {
            return Ok(self.afloat_size.as_ref().unwrap());
        }
        self.afloat_size = Some(self.afloat.len() as i32);
        return Ok(self.afloat_size.as_ref().unwrap());
    }
    fn astr_size<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.astr_size.is_some() {
            return Ok(self.astr_size.as_ref().unwrap());
        }
        self.astr_size = Some(self.astr.len() as i32);
        return Ok(self.astr_size.as_ref().unwrap());
    }
    fn aint_min<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u32> {
        if self.aint_min.is_some() {
            return Ok(self.aint_min.as_ref().unwrap());
        }
        self.aint_min = Some(self.aint.iter().min() as u32);
        return Ok(self.aint_min.as_ref().unwrap());
    }
    fn afloat_min<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&f64> {
        if self.afloat_min.is_some() {
            return Ok(self.afloat_min.as_ref().unwrap());
        }
        self.afloat_min = Some(self.afloat.iter().min() as f64);
        return Ok(self.afloat_min.as_ref().unwrap());
    }
    fn aint_size<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&i32> {
        if self.aint_size.is_some() {
            return Ok(self.aint_size.as_ref().unwrap());
        }
        self.aint_size = Some(self.aint.len() as i32);
        return Ok(self.aint_size.as_ref().unwrap());
    }
    fn aint_last<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u32> {
        if self.aint_last.is_some() {
            return Ok(self.aint_last.as_ref().unwrap());
        }
        self.aint_last = Some(self.aint.last() as u32);
        return Ok(self.aint_last.as_ref().unwrap());
    }
    fn afloat_last<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&f64> {
        if self.afloat_last.is_some() {
            return Ok(self.afloat_last.as_ref().unwrap());
        }
        self.afloat_last = Some(self.afloat.last() as f64);
        return Ok(self.afloat_last.as_ref().unwrap());
    }
    fn astr_first<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.astr_first.is_some() {
            return Ok(self.astr_first.as_ref().unwrap());
        }
        self.astr_first = Some(self.astr.first().to_string());
        return Ok(self.astr_first.as_ref().unwrap());
    }
    fn astr_last<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.astr_last.is_some() {
            return Ok(self.astr_last.as_ref().unwrap());
        }
        self.astr_last = Some(self.astr.last().to_string());
        return Ok(self.astr_last.as_ref().unwrap());
    }
    fn aint_max<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&u32> {
        if self.aint_max.is_some() {
            return Ok(self.aint_max.as_ref().unwrap());
        }
        self.aint_max = Some(self.aint.iter().max() as u32);
        return Ok(self.aint_max.as_ref().unwrap());
    }
    fn afloat_first<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&f64> {
        if self.afloat_first.is_some() {
            return Ok(self.afloat_first.as_ref().unwrap());
        }
        self.afloat_first = Some(self.afloat.first() as f64);
        return Ok(self.afloat_first.as_ref().unwrap());
    }
    fn astr_min<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.astr_min.is_some() {
            return Ok(self.astr_min.as_ref().unwrap());
        }
        self.astr_min = Some(self.astr.iter().min().to_string());
        return Ok(self.astr_min.as_ref().unwrap());
    }
    fn astr_max<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&String> {
        if self.astr_max.is_some() {
            return Ok(self.astr_max.as_ref().unwrap());
        }
        self.astr_max = Some(self.astr.iter().max().to_string());
        return Ok(self.astr_max.as_ref().unwrap());
    }
    fn afloat_max<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self>,
        _parent: Option<TypedStack<KStructUnit>>
    ) -> KResult<&f64> {
        if self.afloat_max.is_some() {
            return Ok(self.afloat_max.as_ref().unwrap());
        }
        self.afloat_max = Some(self.afloat.iter().max() as f64);
        return Ok(self.afloat_max.as_ref().unwrap());
    }
}
