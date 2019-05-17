// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::switch_manual_enum::*;
use std::fs;

#[test]
fn test_switch_manual_enum() {
    let data = fs::read("../../src/switch_opcodes.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let mut r = SwitchManualEnum::default();
    r.read(&reader, None, KStructUnit::parent_stack()).expect("Unable to parse data.");
    // assert_eq!(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(opcodes)),identifier(size)), IntNum(4))
    // assert_eq!(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(opcodes)),IntNum(0)),identifier(code)), EnumByLabel(identifier(code_enum),identifier(strval),typeId(false,ArrayBuffer(switch_manual_enum, opcode),false)))
    // assert_eq!(Attribute(CastToType(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(opcodes)),IntNum(0)),identifier(body)),typeId(false,ArrayBuffer(switch_manual_enum, opcode, strval),false)),identifier(value)), Str(foobar))
    // assert_eq!(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(opcodes)),IntNum(1)),identifier(code)), EnumByLabel(identifier(code_enum),identifier(intval),typeId(false,ArrayBuffer(switch_manual_enum, opcode),false)))
    // assert_eq!(Attribute(CastToType(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(opcodes)),IntNum(1)),identifier(body)),typeId(false,ArrayBuffer(switch_manual_enum, opcode, intval),false)),identifier(value)), IntNum(66))
    // assert_eq!(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(opcodes)),IntNum(2)),identifier(code)), EnumByLabel(identifier(code_enum),identifier(intval),typeId(false,ArrayBuffer(switch_manual_enum, opcode),false)))
    // assert_eq!(Attribute(CastToType(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(opcodes)),IntNum(2)),identifier(body)),typeId(false,ArrayBuffer(switch_manual_enum, opcode, intval),false)),identifier(value)), IntNum(55))
    // assert_eq!(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(opcodes)),IntNum(3)),identifier(code)), EnumByLabel(identifier(code_enum),identifier(strval),typeId(false,ArrayBuffer(switch_manual_enum, opcode),false)))
    // assert_eq!(Attribute(CastToType(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(opcodes)),IntNum(3)),identifier(body)),typeId(false,ArrayBuffer(switch_manual_enum, opcode, strval),false)),identifier(value)), Str())
}
