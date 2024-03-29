// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::SwitchManualIntSizeEos;

#[test]
fn test_switch_manual_int_size_eos() {
    if let Ok(r) = SwitchManualIntSizeEos::from_file("src/switch_tlv.bin") {

        assert_eq!(r.chunks.len(), 4);
        assert_eq!(r.chunks[0].code, 17);
        assert_eq!(r.chunks[0].body.body.title, "Stuff");
        assert_eq!(r.chunks[0].body.body.author, "Me");
        assert_eq!(r.chunks[1].code, 34);
        assert_eq!(r.chunks[1].body.body.entries, ["AAAA", "BBBB", "CCCC"]);
        assert_eq!(r.chunks[2].code, 51);
        assert_eq!(r.chunks[2].body.body, vec!([0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80]));
        assert_eq!(r.chunks[3].code, 255);
        assert_eq!(r.chunks[3].body.body, vec!([]));
    }
}
