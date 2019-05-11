// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::index_to_param_until::IndexToParamUntil;
use std::fs;

#[test]
fn test_index_to_param_until() {
    let data = fs::read("src/index_sizes.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = IndexToParamUntil::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.qty, 3);
    // assert_eq!(r.sizes[0], 1);
    // assert_eq!(r.sizes[1], 8);
    // assert_eq!(r.sizes[2], 4);
    // assert_eq!(r.blocks[0].buf, "A");
    // assert_eq!(r.blocks[1].buf, "BBBBBBBB");
    // assert_eq!(r.blocks[2].buf, "CCCC");
}
