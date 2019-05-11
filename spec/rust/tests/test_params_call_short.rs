// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::params_call_short::ParamsCallShort;
use std::fs;

#[test]
fn test_params_call_short() {
    let data = fs::read("src/term_strz.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = ParamsCallShort::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.buf1.body, "foo|b");
    // assert_eq!(r.buf2.body, "ar|ba");
    // assert_eq!(r.buf2.trailer, 122);
}
