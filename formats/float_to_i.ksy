meta:
  id: float_to_i
  endian: le
seq:
  - id: single_value
    type: f4
  - id: double_value
    type: f8
  - id: single_value_if
    type: f4be
    if: true
  - id: double_value_if
    type: f8be
    if: true
instances:
  calc_float1:
    value: 1.234
  calc_float2:
    value: 1.5
  calc_float3:
    value: 1.9
  calc_float4:
    value: -2.7
  calc_if:
    value: 13.9
  single_i:
    value: single_value.to_i
  double_i:
    value: double_value.to_i
  single_if_i:
    value: single_value_if.to_i
  double_if_i:
    value: double_value_if.to_i
  float1_i:
    value: calc_float1.to_i
  float2_i:
    value: calc_float2.to_i
  float3_i:
    value: calc_float3.to_i
  float4_i:
    value: calc_float4.to_i
  calc_if_i:
    value: calc_if.to_i
