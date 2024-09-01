extern crate kaitai;
use kaitai::CustomDecoder;

pub struct MyCustomFx {
    key: i32,
}

impl MyCustomFx {
    pub fn new(p_key: u8, p_flag: bool, _p_some_bytes: &[u8]) -> Self {
        if p_flag {
            Self { key: p_key as i32 }
        } else {
            Self {
                key: -(p_key as i32),
            }
        }
    }
}

impl CustomDecoder for MyCustomFx {
    fn decode(&self, bytes: &[u8]) -> Result<Vec<u8>, String> {
        let mut res = bytes.to_vec();
        for i in res.iter_mut() {
            *i = (*i as i32 + self.key) as u8;
        }
        Ok(res)
    }
}
