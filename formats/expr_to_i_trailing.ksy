meta:
  id: expr_to_i_trailing
  encoding: ASCII
instances:
  to_i_r10:
    value: '"9173abc".to_i'
  to_i_r16:
    value: '"9173abc".to_i(16)'
  to_i_garbage:
    value: '"123_.^".to_i'
