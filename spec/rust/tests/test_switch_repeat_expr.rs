use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::switch_repeat_expr::*;


#[test]
fn test_switch_repeat_expr() {
    let bytes = fs::read("../../src/switch_tlv.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = SwitchRepeatExpr::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {
        panic!("{:?}", err);
    }

    assert_eq!(*r.code(), 17);
    assert_eq!(*r.size(), 9);

    if let SwitchRepeatExpr_Body::SwitchRepeatExpr_One(b) =  &r.body()[0 as usize] {
        assert_eq!(b.first(), &vec![0x53u8, 0x74u8, 0x75u8, 0x66u8, 0x66u8, 0x0u8, 0x4du8, 0x65u8, 0x0u8]);
    } else {
        panic!("expected enum SwitchRepeatExpr_Body");
    }
}
