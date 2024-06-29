use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::params_def::*;

#[test]
fn test_params_def() -> KResult<()> {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let f = |t: &mut ParamsDef| Ok(t.set_params(5, true));
    let r: OptRc<ParamsDef> = ParamsDef::read_into_with_init(&_io, None, None, &f)?;

    assert_eq!(*r.buf(), "foo|b");
    assert_eq!(*r.trailer(), 0x61);
    Ok(())
}
