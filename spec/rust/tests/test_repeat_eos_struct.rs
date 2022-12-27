use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::repeat_eos_struct::*;


#[test]
fn test_repeat_eos_struct() {
    let bytes = fs::read("../../src/repeat_eos_struct.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let res = RepeatEosStruct::read_into(&reader, None, None);
    let r : Rc<RepeatEosStruct>;

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
