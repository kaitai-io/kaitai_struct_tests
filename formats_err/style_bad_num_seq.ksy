# style_bad_num_seq.ksy: /seq/0/id:
# 	warning: use `num_foos` instead of `size_of_foo`, given that it's only used as repeat count of `foos` (see https://doc.kaitai.io/ksy_style_guide.html#attr-id)
#
meta:
  id: style_bad_num_seq
seq:
  - id: size_of_foo
    type: u1
  - id: foos
    type: u1
    repeat: expr
    repeat-expr: size_of_foo
