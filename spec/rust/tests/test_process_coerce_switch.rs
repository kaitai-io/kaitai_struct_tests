// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::process_coerce_switch::ProcessCoerceSwitch;
use std::fs;

#[test]
fn test_process_coerce_switch() {
    let data = fs::read("../../src/process_coerce_switch.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = ProcessCoerceSwitch::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.buf_type, 0);
    // assert_eq!(r.flag, 0);
    // assert_eq!(r.buf.bar, vec!([0x41, 0x41, 0x41, 0x41]));
}
