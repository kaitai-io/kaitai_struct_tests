// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate kaitai_rust;

use kaitai_struct::KaitaiStruct;
use rust::EnumIf;

#[test]
fn test_enum_if() {
    if let Ok(r) = EnumIf::from_file("src/if_struct.bin") {

        assert_eq!(r.op1.opcode, EnumIf_Opcodes::AString);
        assert_eq!(r.op1.arg_str.str, "foo");
        assert_eq!(r.op2.opcode, EnumIf_Opcodes::ATuple);
        assert_eq!(r.op2.arg_tuple.num1, 66);
        assert_eq!(r.op2.arg_tuple.num2, 67);
        assert_eq!(r.op3.opcode, EnumIf_Opcodes::AString);
        assert_eq!(r.op3.arg_str.str, "bar");
    }
}
