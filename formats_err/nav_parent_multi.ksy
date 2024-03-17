# nav_parent_multi.ksy: /types/common/seq/0/size:
# 	error: don't know how to call method 'len_body' of object type 'CalcKaitaiStructType(false)'
#
meta:
  id: nav_parent_multi
seq:
  - id: len_body
    type: u1
  - id: branch
    type: common
  - id: mess_up
    type: foo
types:
  foo:
    seq:
      - id: branch
        type: common
  common:
    seq:
      - id: body
        size: _parent.len_body
