# seq_dup_id.ksy: /seq/2: error: duplicate attribute ID 'foo', previously defined at /seq/0
meta:
  id: seq_dup_id
seq:
  - id: foo
    type: u1
  - id: bar
    type: u1
  - id: foo
    size: 5
