// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::process_coerce_bytes::ProcessCoerceBytes;
use std::fs;

#[test]
fn test_process_coerce_bytes() {
    let data = fs::read("src/process_coerce_bytes.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = ProcessCoerceBytes::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.records[0].flag, 0);
    // assert_eq!(r.records[0].buf, vec!([0x41, 0x41, 0x41, 0x41]));
    // assert_eq!(r.records[1].flag, 1);
    // assert_eq!(r.records[1].buf, vec!([0x42, 0x42, 0x42, 0x42]));
}
