use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::repeat_n_struct::*;


#[test]
fn test_repeat_n_struct() {
    let bytes = fs::read("../../src/repeat_n_struct.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = RepeatNStruct::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {
        panic!("{:?}", err);
    }

    assert_eq!(r.chunks().len(), 2);
    assert_eq!(*r.chunks()[0 as usize].offset(), 16);
    assert_eq!(*r.chunks()[0 as usize].len(), 8312);
    assert_eq!(*r.chunks()[1 as usize].offset(), 8328);
    assert_eq!(*r.chunks()[1 as usize].len(), 15);
}
