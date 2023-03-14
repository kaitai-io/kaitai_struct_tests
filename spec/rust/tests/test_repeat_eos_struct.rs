use std::fs;

extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::repeat_eos_struct::*;


#[test]
fn test_repeat_eos_struct() {
    let bytes = fs::read("../../src/repeat_eos_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res = RepeatEosStruct::read_into(&_io, None, None);
    let r : OptRc<RepeatEosStruct>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(r.chunks().len(), 2);
    assert_eq!(*r.chunks()[0 as usize].offset(), 0);
    assert_eq!(*r.chunks()[0 as usize].len(), 66);
    assert_eq!(*r.chunks()[1 as usize].offset(), 66);
    assert_eq!(*r.chunks()[1 as usize].len(), 2069);
}
