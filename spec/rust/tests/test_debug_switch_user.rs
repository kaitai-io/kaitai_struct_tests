#![allow(dead_code)]
use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::debug_switch_user::*;

#[test]
fn basic_parse() {
    let bytes = fs::read("../../src/nav_parent_switch.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let res = DebugSwitchUser::read_into(&reader, None, None);
    let r : Rc<DebugSwitchUser>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.code(), 1);
    if let DebugSwitchUser_Data::DebugSwitchUser_One(s) = r.data().as_ref().unwrap() {
        assert_eq!(*s.val(), -190);
    } else {
        panic!("expected enum DebugSwitchUser_Data");
    };
}
