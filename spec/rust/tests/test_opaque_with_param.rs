use std::fs;

extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::opaque_with_param::*;


#[test]
fn test_opaque_with_param() {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res = OpaqueWithParam::read_into(&_io, None, None);
    let r : OptRc<OpaqueWithParam>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    let one = r.one();
    let p = one;
    assert_eq!(*p.buf(), "foo|b");
    assert_eq!(*p.trailer(), 0x61);
}
