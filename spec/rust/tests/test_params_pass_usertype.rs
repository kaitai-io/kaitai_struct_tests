// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::params_pass_usertype::ParamsPassUsertype;
use std::fs;

#[test]
fn test_params_pass_usertype() {
    let data = fs::read("../../src/position_in_seq.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = ParamsPassUsertype::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.first.foo, 1);
    // assert_eq!(r.one.buf, vec!([0x2]));
}
