// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::enum_import_seq::EnumImportSeq;

#[test]
fn test_enum_import_seq() {
    let r = EnumImportSeq::from_file("../../src/enum_0.bin").expect("file for parsing is not found");

    assert_eq!(r.pet_1, Enum0__Animal::CAT);
    assert_eq!(r.pet_2, EnumDeep__Container1__Container2__Animal::HARE);
}
