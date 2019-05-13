// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::zlib_with_header_78::ZlibWithHeader78;
use std::fs;

#[test]
fn test_zlib_with_header_78() {
    let data = fs::read("../../src/zlib_with_header_78.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = ZlibWithHeader78::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.data, vec!([0x61, 0x20, 0x71, 0x75, 0x69, 0x63, 0x6b, 0x20, 0x62, 0x72, 0x6f, 0x77, 0x6e, 0x20, 0x66, 0x6f, 0x78, 0x20, 0x6a, 0x75, 0x6d, 0x70, 0x73, 0x20, 0x6f, 0x76, 0x65, 0x72]));
}
