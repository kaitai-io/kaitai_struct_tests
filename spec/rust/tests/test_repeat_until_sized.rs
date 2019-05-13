// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::repeat_until_sized::RepeatUntilSized;
use std::fs;

#[test]
fn test_repeat_until_sized() {
    let data = fs::read("../../src/repeat_until_process.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = RepeatUntilSized::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.records.len(), 3);
    // assert_eq!(r.records[0].marker, 232);
    // assert_eq!(r.records[0].body, 2863311546);
    // assert_eq!(r.records[1].marker, 250);
    // assert_eq!(r.records[1].body, 2863315102);
    // assert_eq!(r.records[2].marker, 170);
    // assert_eq!(r.records[2].body, 1431655765);
}
