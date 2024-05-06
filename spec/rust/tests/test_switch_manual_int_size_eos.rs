#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::switch_manual_int_size_eos::*;

#[test]
fn test_switch_manual_int_size_eos() {
    let bytes = fs::read("../../src/switch_tlv.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<SwitchManualIntSizeEos>> =
        SwitchManualIntSizeEos::read_into(&_io, None, None);
    let r: OptRc<SwitchManualIntSizeEos>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(r.chunks().len(), 4);
    assert_eq!(*r.chunks()[0 as usize].code(), 17);
    if let Some(
        SwitchManualIntSizeEos_ChunkBody_Body::SwitchManualIntSizeEos_ChunkBody_ChunkMeta(s),
    ) = r.chunks()[0].body().clone().body().as_ref()
    {
        assert_eq!("Stuff", *s.title());
        assert_eq!("Me", *s.author());
    } else {
        panic!("expected enum SwitchManualIntSizeEos_ChunkBody_ChunkMeta");
    };
    assert_eq!(*r.chunks()[1 as usize].code(), 34);
    if let Some(SwitchManualIntSizeEos_ChunkBody_Body::SwitchManualIntSizeEos_ChunkBody_ChunkDir(
        s,
    )) = r.chunks()[1].body().clone().body().as_ref()
    {
        let strings: Vec<String> = vec!["AAAA", "BBBB", "CCCC"]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        assert_eq!(strings, *s.entries());
    } else {
        panic!("expected enum SwitchManualIntSizeEos_ChunkBody_ChunkDir");
    }
    assert_eq!(*r.chunks()[2 as usize].code(), 51);
    if let Some(SwitchManualIntSizeEos_ChunkBody_Body::Bytes(s)) =
        r.chunks()[2].body().clone().body().as_ref()
    {
        let raw: Vec<u8> = vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80];
        assert_eq!(raw, *s);
    } else {
        panic!("expected enum Bytes");
    }
    assert_eq!(*r.chunks()[3 as usize].code(), 255);
    if let Some(SwitchManualIntSizeEos_ChunkBody_Body::Bytes(s)) =
        r.chunks()[3].body().clone().body().as_ref()
    {
        let raw: Vec<u8> = vec![];
        assert_eq!(raw, *s);
    } else {
        panic!("expected enum Bytes");
    };
}
