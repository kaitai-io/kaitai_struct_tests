// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::position_in_seq::PositionInSeq;
use std::fs;

#[test]
fn test_position_in_seq() {
    let data = fs::read("src/position_in_seq.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = PositionInSeq::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.numbers, [(0 + 1), 2, 3]);
}
