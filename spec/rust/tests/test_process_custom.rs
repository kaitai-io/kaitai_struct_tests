// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::process_custom::ProcessCustom;
use std::fs;

#[test]
fn test_process_custom() {
    let data = fs::read("../../src/process_rotate.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = ProcessCustom::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.buf1, vec!([0x10, 0xb3, 0x94, 0x94, 0xf4]));
    // assert_eq!(r.buf2, vec!([0x5f, 0xba, 0x7b, 0x93, 0x63, 0x23, 0x5f]));
    // assert_eq!(r.buf3, vec!([0x29, 0x33, 0xb1, 0x38, 0xb1]));
}
