// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::imports0::Imports0;
use std::fs;

#[test]
fn test_imports0() {
    let data = fs::read("src/fixed_struct.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = Imports0::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.two, 80);
    // assert_eq!(r.hw.one, 65);
    // assert_eq!(r.hw_one, 65);
}
