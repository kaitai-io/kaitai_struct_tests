// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::process_to_user::ProcessToUser;
use std::fs;

#[test]
fn test_process_to_user() {
    let data = fs::read("../../src/process_rotate.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = ProcessToUser::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.buf1.str, "Hello");
}
