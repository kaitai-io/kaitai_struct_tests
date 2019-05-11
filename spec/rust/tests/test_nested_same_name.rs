// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::nested_same_name::NestedSameName;
use std::fs;

#[test]
fn test_nested_same_name() {
    let data = fs::read("src/repeat_n_struct.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = NestedSameName::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.main_data.main_size, 2);
    // assert_eq!(r.main_data.foo.data, vec!([0x10, 0x0, 0x0, 0x0]));
}
