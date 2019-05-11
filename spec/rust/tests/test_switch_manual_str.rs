// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::switch_manual_str::SwitchManualStr;
use std::fs;

#[test]
fn test_switch_manual_str() {
    let data = fs::read("src/switch_opcodes.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = SwitchManualStr::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.opcodes.len(), 4);
    // assert_eq!(r.opcodes[0].code, "S");
    // assert_eq!(r.opcodes[0].body.value, "foobar");
    // assert_eq!(r.opcodes[1].code, "I");
    // assert_eq!(r.opcodes[1].body.value, 66);
    // assert_eq!(r.opcodes[2].code, "I");
    // assert_eq!(r.opcodes[2].body.value, 55);
    // assert_eq!(r.opcodes[3].code, "S");
    // assert_eq!(r.opcodes[3].body.value, "");
}
