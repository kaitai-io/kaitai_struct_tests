# style_bad_len_seq.ksy: /seq/0/id:
# 	warning: use `len_foo` instead of `size_of_foo`, given that it's only used as a byte size of `foo` (see https://doc.kaitai.io/ksy_style_guide.html#attr-id)
#
meta:
  id: style_bad_len_seq
seq:
  - id: size_of_foo
    type: u1
  - id: foo
    size: size_of_foo
