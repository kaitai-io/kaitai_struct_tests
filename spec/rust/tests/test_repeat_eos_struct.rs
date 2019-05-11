// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::repeat_eos_struct::RepeatEosStruct;
use std::fs;

#[test]
fn test_repeat_eos_struct() {
    let data = fs::read("src/repeat_eos_struct.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = RepeatEosStruct::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.chunks.len(), 2);
    // assert_eq!(r.chunks[0].offset, 0);
    // assert_eq!(r.chunks[0].len, 66);
    // assert_eq!(r.chunks[1].offset, 66);
    // assert_eq!(r.chunks[1].len, 2069);
}
