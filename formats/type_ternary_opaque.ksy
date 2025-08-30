meta:
  id: type_ternary_opaque
  ks-opaque-types: true
seq:
  - id: dif_wo_hack
    size: 1
    type: hello_world
    if: not is_hack
  - id: dif_with_hack
    size: 1
    type: hello_world
    process: xor(0b00000011)
    if: is_hack
instances:
  is_hack:
    value: "false"
  dif:
    value: "not is_hack ? dif_wo_hack : dif_with_hack"
