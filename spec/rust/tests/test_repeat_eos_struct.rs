#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::repeat_eos_struct::*;

#[test]
fn test_repeat_eos_struct() {
    let bytes = fs::read("../../src/repeat_eos_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<RepeatEosStruct>> = RepeatEosStruct::read_into(&_io, None, None);
    let r: OptRc<RepeatEosStruct>;

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
