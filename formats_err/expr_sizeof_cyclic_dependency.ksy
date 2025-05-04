# expr_sizeof_cyclic_dependency.ksy: /seq/0/if:
# 	error: unable to derive sizeof for type `expr_sizeof_cyclic_dependency`: dynamic sized type
#
meta:
  id: expr_sizeof_cyclic_dependency
  -gh-issue: https://github.com/kaitai-io/kaitai_struct/issues/927
seq:
  - id: value
    size: 2
    if: _sizeof == 1
