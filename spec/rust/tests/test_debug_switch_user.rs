#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::debug_switch_user::*;

#[test]
fn basic_parse() {
    let bytes = fs::read("../../src/nav_parent_switch.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = DebugSwitchUser::default();
    {
        let res = r.read(&reader, None, Some(KStructUnit::parent_stack()));
        assert!(res.is_ok());
    }

    assert_eq!(*r.code(), 1);
    if let DebugSwitchUser_Data::DebugSwitchUser_One(s) =  r.data() {
        assert_eq!(*s.val(), -190);
    } else {
        panic!("expected enum DebugSwitchUser_Data");
    }
}
