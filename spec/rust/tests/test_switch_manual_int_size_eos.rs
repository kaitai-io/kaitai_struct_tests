#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::switch_manual_int_size_eos::*;

#[test]
fn test_switch_manual_int_size_eos() {
    let bytes = fs::read("../../src/switch_tlv.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = SwitchManualIntSizeEos::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {

        panic!("{:?}", err);
    }
    assert_eq!(4, r.chunks().len());

    assert_eq!(17, *r.chunks()[0].code());
    if let SwitchManualIntSizeEos_ChunkBody_Body::SwitchManualIntSizeEos_ChunkBody_ChunkMeta(s) =  r.chunks()[0].body().body() {
        assert_eq!("Stuff", s.title());
        assert_eq!("Me", s.author());
    } else {
        panic!("expected enum SwitchManualIntSizeEos_ChunkBody_ChunkMeta");
    }

    assert_eq!(34, *r.chunks()[1].code());
    if let SwitchManualIntSizeEos_ChunkBody_Body::SwitchManualIntSizeEos_ChunkBody_ChunkDir(s) =  r.chunks()[1].body().body() {
        let strings : Vec<String> = vec!["AAAA", "BBBB", "CCCC"].iter().map(|&s| s.to_string() ).collect();
        assert_eq!(strings, *s.entries());
    } else {
        panic!("expected enum SwitchManualIntSizeEos_ChunkBody_ChunkDir");
    }
 
    assert_eq!(51, *r.chunks()[2].code());
    if let SwitchManualIntSizeEos_ChunkBody_Body::Bytes(s) = r.chunks()[2].body().body() {
        let raw : Vec<u8> = vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80];
        assert_eq!(raw, *s);
    } else {
        panic!("expected enum Bytes");
    }
 
    assert_eq!(255, *r.chunks()[3].code());
    if let SwitchManualIntSizeEos_ChunkBody_Body::Bytes(s) =  r.chunks()[3].body().body() {
        let raw : Vec<u8> = vec![];
        assert_eq!(raw, *s);
    } else {
        panic!("expected enum Bytes");
    };
}
