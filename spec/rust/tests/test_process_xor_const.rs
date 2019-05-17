// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::process_xor_const::*;
use std::fs;

#[test]
fn test_process_xor_const() {
    let data = fs::read("../../src/process_xor_1.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let mut r = ProcessXorConst::default();
    r.read(&reader, None, KStructUnit::parent_stack()).expect("Unable to parse data.");
    assert_eq!(r.key, 255);
    assert_eq!(r.buf, &[0x66, 0x6f, 0x6f, 0x20, 0x62, 0x61, 0x72]);
}
