# style_bad_num_inst_pos.ksy: /instances/size_of_foo:
# 	warning: use `num_foos` instead of `size_of_foo`, given that it's only used as repeat count of `foos` (see https://doc.kaitai.io/ksy_style_guide.html#attr-id)
#
meta:
  id: style_bad_num_inst_pos
seq:
  - id: foos
    type: u1
    repeat: expr
    repeat-expr: size_of_foo
instances:
  size_of_foo:
    pos: 4
    type: u1
