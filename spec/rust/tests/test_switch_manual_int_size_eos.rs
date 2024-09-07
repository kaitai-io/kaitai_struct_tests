// Autogenerated from KST: please remove this line if doing any edits by hand!

use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::switch_manual_int_size_eos::*;

#[test]
fn test_switch_manual_int_size_eos() -> KResult<()> {
    let bytes = fs::read("../../src/switch_tlv.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<SwitchManualIntSizeEos> = SwitchManualIntSizeEos::read_into(&_io, None, None)?;

    assert_eq!(r.chunks().len(), 4);
    assert_eq!(*r.chunks()[0 as usize].code(), 17);
    assert_eq!(*Into::<OptRc<SwitchManualIntSizeEos_ChunkBody_ChunkMeta>>::into(&*r.chunks()[0 as usize].body().body().as_ref().unwrap()).title(), "Stuff");
    assert_eq!(*Into::<OptRc<SwitchManualIntSizeEos_ChunkBody_ChunkMeta>>::into(&*r.chunks()[0 as usize].body().body().as_ref().unwrap()).author(), "Me");
    assert_eq!(*r.chunks()[1 as usize].code(), 34);
    assert_eq!(*Into::<OptRc<SwitchManualIntSizeEos_ChunkBody_ChunkDir>>::into(&*r.chunks()[1 as usize].body().body().as_ref().unwrap()).entries(), vec!["AAAA".to_string(), "BBBB".to_string(), "CCCC".to_string()]);
    assert_eq!(*r.chunks()[2 as usize].code(), 51);
    assert_eq!(Into::<Vec<u8>>::into(&*r.chunks()[2 as usize].body().body().as_ref().unwrap()), vec![0x10u8, 0x20u8, 0x30u8, 0x40u8, 0x50u8, 0x60u8, 0x70u8, 0x80u8]);
    assert_eq!(*r.chunks()[3 as usize].code(), 255);
    assert_eq!(Into::<Vec<u8>>::into(&*r.chunks()[3 as usize].body().body().as_ref().unwrap()), vec![]);
    Ok(())
}
