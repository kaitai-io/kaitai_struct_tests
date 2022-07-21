// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
// extraAttrForIO(RawIdentifier(NamedIdentifier(ltr)), NoRepeat)
// extraAttrForIO(RawIdentifier(NamedIdentifier(rtl)), NoRepeat)
// extraAttrForIO(RawIdentifier(NamedIdentifier(leading_zero_ltr)), NoRepeat)

#[derive(Default, Debug, PartialEq)]
pub struct BcdUserTypeLe {
    pub ltr: Option<BcdUserTypeLe_LtrObj>,
    pub rtl: Option<BcdUserTypeLe_RtlObj>,
    pub leading_zero_ltr: Option<BcdUserTypeLe_LeadingZeroLtrObj>,
    pub raw_ltr: Vec<u8>,
    pub raw_rtl: Vec<u8>,
    pub raw_leading_zero_ltr: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for BcdUserTypeLe {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.ltr = Some(Self::read_into::<BytesReader, BcdUserTypeLe_LtrObj>(&BytesReader::new(_io.read_bytes(4 as usize)?), Some(self), _parent.push(self))?);
        self.rtl = Some(Self::read_into::<BytesReader, BcdUserTypeLe_RtlObj>(&BytesReader::new(_io.read_bytes(4 as usize)?), Some(self), _parent.push(self))?);
        self.leading_zero_ltr = Some(Self::read_into::<BytesReader, BcdUserTypeLe_LeadingZeroLtrObj>(&BytesReader::new(_io.read_bytes(4 as usize)?), Some(self), _parent.push(self))?);
        Ok(())
    }
}
impl<'r, 's: 'r> BcdUserTypeLe {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = BcdUserTypeLe::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}

#[derive(Default, Debug, PartialEq)]
pub struct BcdUserTypeLe_LtrObj {
    pub b1: u8,
    pub b2: u8,
    pub b3: u8,
    pub b4: u8,
    pub as_int: Option<i32>,
    pub digit2: Option<i32>,
    pub digit4: Option<i32>,
    pub digit3: Option<i32>,
    pub digit5: Option<i32>,
    pub digit8: Option<i32>,
    pub digit6: Option<i32>,
    pub as_str: Option<String>,
    pub digit1: Option<i32>,
    pub digit7: Option<i32>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for BcdUserTypeLe_LtrObj {
    type Root = BcdUserTypeLe;
    type ParentStack = (&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.b1 = _io.read_u1()?;
        self.b2 = _io.read_u1()?;
        self.b3 = _io.read_u1()?;
        self.b4 = _io.read_u1()?;
        Ok(())
    }
}
impl<'r, 's: 'r> BcdUserTypeLe_LtrObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = BcdUserTypeLe_LtrObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn as_int<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.as_int.is_some() {
            return Ok(self.as_int.as_ref().unwrap());
        }
        self.as_int = Some(((((((((self.digit8(_io, _root, _parent)? * 1) + (self.digit7(_io, _root, _parent)? * 10)) + (self.digit6(_io, _root, _parent)? * 100)) + (self.digit5(_io, _root, _parent)? * 1000)) + (self.digit4(_io, _root, _parent)? * 10000)) + (self.digit3(_io, _root, _parent)? * 100000)) + (self.digit2(_io, _root, _parent)? * 1000000)) + (self.digit1(_io, _root, _parent)? * 10000000)) as i32);
        return Ok(self.as_int.as_ref().unwrap());
    }
    fn digit2<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.digit2.is_some() {
            return Ok(self.digit2.as_ref().unwrap());
        }
        self.digit2 = Some((self.b4 & 15) as i32);
        return Ok(self.digit2.as_ref().unwrap());
    }
    fn digit4<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.digit4.is_some() {
            return Ok(self.digit4.as_ref().unwrap());
        }
        self.digit4 = Some((self.b3 & 15) as i32);
        return Ok(self.digit4.as_ref().unwrap());
    }
    fn digit3<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.digit3.is_some() {
            return Ok(self.digit3.as_ref().unwrap());
        }
        self.digit3 = Some(((self.b3 & 240) >> 4) as i32);
        return Ok(self.digit3.as_ref().unwrap());
    }
    fn digit5<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.digit5.is_some() {
            return Ok(self.digit5.as_ref().unwrap());
        }
        self.digit5 = Some(((self.b2 & 240) >> 4) as i32);
        return Ok(self.digit5.as_ref().unwrap());
    }
    fn digit8<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.digit8.is_some() {
            return Ok(self.digit8.as_ref().unwrap());
        }
        self.digit8 = Some((self.b1 & 15) as i32);
        return Ok(self.digit8.as_ref().unwrap());
    }
    fn digit6<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.digit6.is_some() {
            return Ok(self.digit6.as_ref().unwrap());
        }
        self.digit6 = Some((self.b2 & 15) as i32);
        return Ok(self.digit6.as_ref().unwrap());
    }
    fn as_str<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&String> {
        if self.as_str.is_some() {
            return Ok(self.as_str.as_ref().unwrap());
        }
        self.as_str = Some(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", self.digit1(_io, _root, _parent)?.to_string(), self.digit2(_io, _root, _parent)?.to_string()), self.digit3(_io, _root, _parent)?.to_string()), self.digit4(_io, _root, _parent)?.to_string()), self.digit5(_io, _root, _parent)?.to_string()), self.digit6(_io, _root, _parent)?.to_string()), self.digit7(_io, _root, _parent)?.to_string()), self.digit8(_io, _root, _parent)?.to_string()).to_string());
        return Ok(self.as_str.as_ref().unwrap());
    }
    fn digit1<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.digit1.is_some() {
            return Ok(self.digit1.as_ref().unwrap());
        }
        self.digit1 = Some(((self.b4 & 240) >> 4) as i32);
        return Ok(self.digit1.as_ref().unwrap());
    }
    fn digit7<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.digit7.is_some() {
            return Ok(self.digit7.as_ref().unwrap());
        }
        self.digit7 = Some(((self.b1 & 240) >> 4) as i32);
        return Ok(self.digit7.as_ref().unwrap());
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct BcdUserTypeLe_RtlObj {
    pub b1: u8,
    pub b2: u8,
    pub b3: u8,
    pub b4: u8,
    pub as_int: Option<i32>,
    pub digit2: Option<i32>,
    pub digit4: Option<i32>,
    pub digit3: Option<i32>,
    pub digit5: Option<i32>,
    pub digit8: Option<i32>,
    pub digit6: Option<i32>,
    pub as_str: Option<String>,
    pub digit1: Option<i32>,
    pub digit7: Option<i32>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for BcdUserTypeLe_RtlObj {
    type Root = BcdUserTypeLe;
    type ParentStack = (&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.b1 = _io.read_u1()?;
        self.b2 = _io.read_u1()?;
        self.b3 = _io.read_u1()?;
        self.b4 = _io.read_u1()?;
        Ok(())
    }
}
impl<'r, 's: 'r> BcdUserTypeLe_RtlObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = BcdUserTypeLe_RtlObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn as_int<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.as_int.is_some() {
            return Ok(self.as_int.as_ref().unwrap());
        }
        self.as_int = Some(((((((((self.digit1(_io, _root, _parent)? * 1) + (self.digit2(_io, _root, _parent)? * 10)) + (self.digit3(_io, _root, _parent)? * 100)) + (self.digit4(_io, _root, _parent)? * 1000)) + (self.digit5(_io, _root, _parent)? * 10000)) + (self.digit6(_io, _root, _parent)? * 100000)) + (self.digit7(_io, _root, _parent)? * 1000000)) + (self.digit8(_io, _root, _parent)? * 10000000)) as i32);
        return Ok(self.as_int.as_ref().unwrap());
    }
    fn digit2<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.digit2.is_some() {
            return Ok(self.digit2.as_ref().unwrap());
        }
        self.digit2 = Some((self.b4 & 15) as i32);
        return Ok(self.digit2.as_ref().unwrap());
    }
    fn digit4<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.digit4.is_some() {
            return Ok(self.digit4.as_ref().unwrap());
        }
        self.digit4 = Some((self.b3 & 15) as i32);
        return Ok(self.digit4.as_ref().unwrap());
    }
    fn digit3<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.digit3.is_some() {
            return Ok(self.digit3.as_ref().unwrap());
        }
        self.digit3 = Some(((self.b3 & 240) >> 4) as i32);
        return Ok(self.digit3.as_ref().unwrap());
    }
    fn digit5<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.digit5.is_some() {
            return Ok(self.digit5.as_ref().unwrap());
        }
        self.digit5 = Some(((self.b2 & 240) >> 4) as i32);
        return Ok(self.digit5.as_ref().unwrap());
    }
    fn digit8<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.digit8.is_some() {
            return Ok(self.digit8.as_ref().unwrap());
        }
        self.digit8 = Some((self.b1 & 15) as i32);
        return Ok(self.digit8.as_ref().unwrap());
    }
    fn digit6<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.digit6.is_some() {
            return Ok(self.digit6.as_ref().unwrap());
        }
        self.digit6 = Some((self.b2 & 15) as i32);
        return Ok(self.digit6.as_ref().unwrap());
    }
    fn as_str<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&String> {
        if self.as_str.is_some() {
            return Ok(self.as_str.as_ref().unwrap());
        }
        self.as_str = Some(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", self.digit8(_io, _root, _parent)?.to_string(), self.digit7(_io, _root, _parent)?.to_string()), self.digit6(_io, _root, _parent)?.to_string()), self.digit5(_io, _root, _parent)?.to_string()), self.digit4(_io, _root, _parent)?.to_string()), self.digit3(_io, _root, _parent)?.to_string()), self.digit2(_io, _root, _parent)?.to_string()), self.digit1(_io, _root, _parent)?.to_string()).to_string());
        return Ok(self.as_str.as_ref().unwrap());
    }
    fn digit1<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.digit1.is_some() {
            return Ok(self.digit1.as_ref().unwrap());
        }
        self.digit1 = Some(((self.b4 & 240) >> 4) as i32);
        return Ok(self.digit1.as_ref().unwrap());
    }
    fn digit7<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.digit7.is_some() {
            return Ok(self.digit7.as_ref().unwrap());
        }
        self.digit7 = Some(((self.b1 & 240) >> 4) as i32);
        return Ok(self.digit7.as_ref().unwrap());
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct BcdUserTypeLe_LeadingZeroLtrObj {
    pub b1: u8,
    pub b2: u8,
    pub b3: u8,
    pub b4: u8,
    pub as_int: Option<i32>,
    pub digit2: Option<i32>,
    pub digit4: Option<i32>,
    pub digit3: Option<i32>,
    pub digit5: Option<i32>,
    pub digit8: Option<i32>,
    pub digit6: Option<i32>,
    pub as_str: Option<String>,
    pub digit1: Option<i32>,
    pub digit7: Option<i32>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for BcdUserTypeLe_LeadingZeroLtrObj {
    type Root = BcdUserTypeLe;
    type ParentStack = (&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.b1 = _io.read_u1()?;
        self.b2 = _io.read_u1()?;
        self.b3 = _io.read_u1()?;
        self.b4 = _io.read_u1()?;
        Ok(())
    }
}
impl<'r, 's: 'r> BcdUserTypeLe_LeadingZeroLtrObj {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = BcdUserTypeLe_LeadingZeroLtrObj::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

    fn as_int<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.as_int.is_some() {
            return Ok(self.as_int.as_ref().unwrap());
        }
        self.as_int = Some(((((((((self.digit8(_io, _root, _parent)? * 1) + (self.digit7(_io, _root, _parent)? * 10)) + (self.digit6(_io, _root, _parent)? * 100)) + (self.digit5(_io, _root, _parent)? * 1000)) + (self.digit4(_io, _root, _parent)? * 10000)) + (self.digit3(_io, _root, _parent)? * 100000)) + (self.digit2(_io, _root, _parent)? * 1000000)) + (self.digit1(_io, _root, _parent)? * 10000000)) as i32);
        return Ok(self.as_int.as_ref().unwrap());
    }
    fn digit2<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.digit2.is_some() {
            return Ok(self.digit2.as_ref().unwrap());
        }
        self.digit2 = Some((self.b4 & 15) as i32);
        return Ok(self.digit2.as_ref().unwrap());
    }
    fn digit4<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.digit4.is_some() {
            return Ok(self.digit4.as_ref().unwrap());
        }
        self.digit4 = Some((self.b3 & 15) as i32);
        return Ok(self.digit4.as_ref().unwrap());
    }
    fn digit3<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.digit3.is_some() {
            return Ok(self.digit3.as_ref().unwrap());
        }
        self.digit3 = Some(((self.b3 & 240) >> 4) as i32);
        return Ok(self.digit3.as_ref().unwrap());
    }
    fn digit5<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.digit5.is_some() {
            return Ok(self.digit5.as_ref().unwrap());
        }
        self.digit5 = Some(((self.b2 & 240) >> 4) as i32);
        return Ok(self.digit5.as_ref().unwrap());
    }
    fn digit8<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.digit8.is_some() {
            return Ok(self.digit8.as_ref().unwrap());
        }
        self.digit8 = Some((self.b1 & 15) as i32);
        return Ok(self.digit8.as_ref().unwrap());
    }
    fn digit6<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.digit6.is_some() {
            return Ok(self.digit6.as_ref().unwrap());
        }
        self.digit6 = Some((self.b2 & 15) as i32);
        return Ok(self.digit6.as_ref().unwrap());
    }
    fn as_str<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&String> {
        if self.as_str.is_some() {
            return Ok(self.as_str.as_ref().unwrap());
        }
        self.as_str = Some(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", self.digit1(_io, _root, _parent)?.to_string(), self.digit2(_io, _root, _parent)?.to_string()), self.digit3(_io, _root, _parent)?.to_string()), self.digit4(_io, _root, _parent)?.to_string()), self.digit5(_io, _root, _parent)?.to_string()), self.digit6(_io, _root, _parent)?.to_string()), self.digit7(_io, _root, _parent)?.to_string()), self.digit8(_io, _root, _parent)?.to_string()).to_string());
        return Ok(self.as_str.as_ref().unwrap());
    }
    fn digit1<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.digit1.is_some() {
            return Ok(self.digit1.as_ref().unwrap());
        }
        self.digit1 = Some(((self.b4 & 240) >> 4) as i32);
        return Ok(self.digit1.as_ref().unwrap());
    }
    fn digit7<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r BcdUserTypeLe>,
        _parent: Option<TypedStack<(&'r BcdUserTypeLe, <BcdUserTypeLe as KStruct<'r, 's>>::ParentStack)>>
    ) -> KResult<&i32> {
        if self.digit7.is_some() {
            return Ok(self.digit7.as_ref().unwrap());
        }
        self.digit7 = Some(((self.b1 & 240) >> 4) as i32);
        return Ok(self.digit7.as_ref().unwrap());
    }
}
