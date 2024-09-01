extern crate kaitai;

#[allow(non_snake_case)]

pub mod Nested {
    pub mod Deeply {
        pub struct CustomFx {}

        impl CustomFx {
            pub fn new(_p_key: u8) -> Self {
                Self {}
            }
        }

        impl kaitai::CustomDecoder for CustomFx {
            fn decode(&self, bytes: &[u8]) -> Result<Vec<u8>, String> {
                let mut res = bytes.to_vec();
                res.insert(0, '_' as u8);
                res.push('_' as u8);
                Ok(res)
            }
        }
    }
}
