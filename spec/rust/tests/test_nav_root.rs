// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::nav_root::NavRoot;
use std::fs;

#[test]
fn test_nav_root() {
    let data = fs::read("src/nav.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = NavRoot::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.header.qty_entries, 2);
    // assert_eq!(r.header.filename_len, 8);
    // assert_eq!(r.index.entries.len(), 2);
    // assert_eq!(r.index.entries[0].filename, "FIRST___");
    // assert_eq!(r.index.entries[1].filename, "SECOND__");
}
