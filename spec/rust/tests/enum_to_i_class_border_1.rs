// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai_struct::KaitaiStruct;
use kaitai_test_suite::enum_to_i_class_border_1::EnumToIClassBorder1;

#[test]
fn test_enum_to_i_class_border_1() {
    let r = EnumToIClassBorder1::from_file("../../src/enum_0.bin").expect("file for parsing is not found");

    assert_eq!(r.pet_1, EnumToIClassBorder1__Animal::CAT);
    assert_eq!(r.pet_2, EnumToIClassBorder1__Animal::CHICKEN);
    assert_eq!(r.checker.is_dog, true);
}
