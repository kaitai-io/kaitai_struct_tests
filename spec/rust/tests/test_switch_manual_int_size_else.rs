#![allow(dead_code)]
use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::switch_manual_int_size_else::*;

#[test]
fn test_switch_manual_int_size_else() {
    let bytes = fs::read("../../src/switch_tlv.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let res = SwitchManualIntSizeElse::read_into(&reader, None, None);
    let r : Rc<SwitchManualIntSizeElse>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }
    assert_eq!(4, r.chunks().len());

    assert_eq!(17, *r.chunks()[0].code());
    if let SwitchManualIntSizeElse_Chunk_Body::SwitchManualIntSizeElse_Chunk_ChunkMeta(s) =  r.chunks()[0].body() {
        assert_eq!("Stuff", s.title());
        assert_eq!("Me", s.author());
    } else {
        panic!("expected enum SwitchManualIntSizeElse_Chunk_ChunkMeta");
    }

    assert_eq!(34, *r.chunks()[1].code());
    if let SwitchManualIntSizeElse_Chunk_Body::SwitchManualIntSizeElse_Chunk_ChunkDir(s) =  r.chunks()[1].body() {
        let strings : Vec<String> = vec!["AAAA", "BBBB", "CCCC"].iter().map(|&s| s.to_string() ).collect();
        assert_eq!(strings, *s.entries());
    } else {
        panic!("expected enum SwitchManualIntSizeElse_Chunk_ChunkDir");
    }
 
    assert_eq!(51, *r.chunks()[2].code());
    if let SwitchManualIntSizeElse_Chunk_Body::SwitchManualIntSizeElse_Chunk_Dummy(s) =  r.chunks()[2].body() {
        let raw : Vec<u8> = vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80];
        assert_eq!(raw, *s.rest());
    } else {
        panic!("expected enum SwitchManualIntSizeElse_Chunk_Dummy");
    }
 
    assert_eq!(255, *r.chunks()[3].code());
    if let SwitchManualIntSizeElse_Chunk_Body::SwitchManualIntSizeElse_Chunk_Dummy(s) =  r.chunks()[3].body() {
        let raw : Vec<u8> = vec![];
        assert_eq!(raw, *s.rest());
    } else {
        panic!("expected enum SwitchManualIntSizeElse_Chunk_Dummy");
    }
}
