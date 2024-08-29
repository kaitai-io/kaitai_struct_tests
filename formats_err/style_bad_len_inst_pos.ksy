# style_bad_len_inst_pos.ksy: /instances/size_of_foo:
# 	warning: use `len_foo` instead of `size_of_foo`, given that it's only used as a byte size of `foo` (see https://doc.kaitai.io/ksy_style_guide.html#attr-id)
#
meta:
  id: style_bad_len_inst_pos
seq:
  - id: foo
    size: size_of_foo
instances:
  size_of_foo:
    pos: 4
    type: u1
