// Autogenerated from KST: please remove this line if doing any edits by hand!

use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::combine_bytes::*;

#[test]
fn test_combine_bytes() -> KResult<()> {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<CombineBytes> = CombineBytes::read_into(&_io, None, None)?;

    assert_eq!(*r.bytes_term(), vec![0x66u8, 0x6fu8, 0x6fu8]);
    assert_eq!(*r.bytes_limit(), vec![0x62u8, 0x61u8, 0x72u8, 0x7cu8]);
    assert_eq!(*r.bytes_eos(), vec![0x62u8, 0x61u8, 0x7au8, 0x40u8]);
    assert_eq!(*r.bytes_calc()?, vec![0x52u8, 0x6eu8, 0x44u8]);
    assert_eq!(*r.term_or_limit()?, vec![0x66u8, 0x6fu8, 0x6fu8]);
    assert_eq!(*r.term_or_eos()?, vec![0x62u8, 0x61u8, 0x7au8, 0x40u8]);
    assert_eq!(*r.term_or_calc()?, vec![0x66u8, 0x6fu8, 0x6fu8]);
    assert_eq!(*r.limit_or_eos()?, vec![0x62u8, 0x61u8, 0x72u8, 0x7cu8]);
    assert_eq!(*r.limit_or_calc()?, vec![0x52u8, 0x6eu8, 0x44u8]);
    assert_eq!(*r.eos_or_calc()?, vec![0x62u8, 0x61u8, 0x7au8, 0x40u8]);
    Ok(())
}
