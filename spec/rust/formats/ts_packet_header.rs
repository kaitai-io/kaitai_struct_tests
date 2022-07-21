// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};
// universalDoc()

#[derive(Default, Debug, PartialEq)]
pub struct TsPacketHeader {
    pub sync_byte: u8,
    pub transport_error_indicator: bool,
    pub payload_unit_start_indicator: bool,
    pub transport_priority: bool,
    pub pid: u64,
    pub transport_scrambling_control: u64,
    pub adaptation_field_control: Option<TsPacketHeader_AdaptationFieldControlEnum>,
    pub continuity_counter: u64,
    pub ts_packet_remain: Vec<u8>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for TsPacketHeader {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.sync_byte = _io.read_u1()?;
        self.transport_error_indicator = _io.read_bits_int(1)? != 0;
        self.payload_unit_start_indicator = _io.read_bits_int(1)? != 0;
        self.transport_priority = _io.read_bits_int(1)? != 0;
        self.pid = _io.read_bits_int(13)?;
        self.transport_scrambling_control = _io.read_bits_int(2)?;
        self.adaptation_field_control = Some((_io.read_bits_int(2)? as i64).try_into()?);
        self.continuity_counter = _io.read_bits_int(4)?;
        _io.align_to_byte()?;
        self.ts_packet_remain = _io.read_bytes(184 as usize)?.to_vec();
        Ok(())
    }
}
impl<'r, 's: 'r> TsPacketHeader {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = TsPacketHeader::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
#[derive(Debug, PartialEq)]
pub enum TsPacketHeader_AdaptationFieldControlEnum {
    Reserved,
    PayloadOnly,
    AdaptationFieldOnly,
    AdaptationFieldAndPayload,
}
impl TryFrom<i64> for TsPacketHeader_AdaptationFieldControlEnum {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<TsPacketHeader_AdaptationFieldControlEnum> {
        match flag {
            0 => Ok(TsPacketHeader_AdaptationFieldControlEnum::Reserved),
            1 => Ok(TsPacketHeader_AdaptationFieldControlEnum::PayloadOnly),
            2 => Ok(TsPacketHeader_AdaptationFieldControlEnum::AdaptationFieldOnly),
            3 => Ok(TsPacketHeader_AdaptationFieldControlEnum::AdaptationFieldAndPayload),
            _ => Err(KError::UnknownVariant(flag)),
        }
    }
}

