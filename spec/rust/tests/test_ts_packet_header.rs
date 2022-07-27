#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::ts_packet_header::*;

#[test]
fn test_ts_packet_header() {
    let bytes = fs::read("../../src/ts_packet.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = TsPacketHeader::default();

    if let Err(err) = r.read(&reader, None, KStructUnit::parent_stack()) {

        panic!("{:?}", err);
    }
    assert_eq!(r.sync_byte(), 71);
    assert_eq!(r.transport_error_indicator(), false);
    assert_eq!(r.payload_unit_start_indicator(), false);
    assert_eq!(r.transport_priority(), true);
    assert_eq!(r.pid(), 33);
    assert_eq!(r.transport_scrambling_control(), 0);
    assert_eq!(*r.adaptation_field_control(), TsPacketHeader_AdaptationFieldControlEnum::PayloadOnly);
}
