extern crate kaitai;
use kaitai::CustomDecoder;

pub struct CustomFxNoArgs {
}

impl CustomFxNoArgs {
    pub fn new() -> Self {
        Self {}
    }
}

impl CustomDecoder for CustomFxNoArgs {
    fn decode(&self, bytes: &[u8]) -> Vec<u8> {
        let mut res = bytes.to_vec();
        res.insert(0, '_' as u8);
        res.push('_' as u8);
        res
    }
}
