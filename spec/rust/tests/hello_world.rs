// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai_struct::KaitaiStruct;
use kaitai_test_suite::hello_world::HelloWorld;

#[test]
fn test_hello_world() {
    let r = HelloWorld::from_file("../../src/fixed_struct.bin").expect("file for parsing is not found");

    assert_eq!(r.one, 80);
}
