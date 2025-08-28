# nav_parent_multi_one_unused.ksy: /types/common/seq/0/size:
# 	error: don't know how to call method 'len_body' of object type 'CalcKaitaiStructType(false)'
#
meta:
  id: nav_parent_multi_one_unused
seq:
  - id: len_body
    type: u1
  - id: branch
    type: common
types:
  # This type is unreachable from the top-level `seq`, but it is still
  # generated, so it must affect the derived type of `_parent` in the
  # `common` type, otherwise we would run into
  # https://github.com/kaitai-io/kaitai_struct/issues/961.
  unreachable:
    seq:
      - id: branch
        type: common
  common:
    seq:
      - id: body
        size: _parent.len_body
