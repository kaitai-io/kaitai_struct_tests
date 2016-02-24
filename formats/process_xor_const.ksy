meta:
  id: process_xor_const
  endian: le
seq:
  - id: key
    type: u1
  - id: buf
    size_eos: true
    process: xor(0xff)
