// Autogenerated from KST: please remove this line if doing any edits by hand!

use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::ts_packet_header::*;

#[test]
fn test_ts_packet_header() -> KResult<()> {
    let bytes = fs::read("../../src/ts_packet.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<TsPacketHeader> = TsPacketHeader::read_into(&_io, None, None)?;

    assert_eq!(*r.sync_byte(), 71);
    assert_eq!(*r.transport_error_indicator(), false);
    assert_eq!(*r.payload_unit_start_indicator(), false);
    assert_eq!(*r.transport_priority(), true);
    assert_eq!(*r.pid(), 33);
    assert_eq!(*r.transport_scrambling_control(), 0);
    assert_eq!(*r.adaptation_field_control(), TsPacketHeader_AdaptationFieldControlEnum::PayloadOnly);
    Ok(())
}
