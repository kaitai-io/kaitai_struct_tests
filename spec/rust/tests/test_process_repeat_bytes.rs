use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::process_repeat_bytes::*;


#[test]
fn test_process_repeat_bytes() {
    let bytes = fs::read("../../src/process_xor_4.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = ProcessRepeatBytes::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {
        panic!("{:?}", err);
    }

    assert_eq!(r.bufs()[0 as usize], vec![0x72u8, 0x25u8, 0x3du8, 0x8au8, 0x14u8]);
    assert_eq!(r.bufs()[1 as usize], vec![0x4au8, 0x52u8, 0xaau8, 0x10u8, 0x44u8]);
}
