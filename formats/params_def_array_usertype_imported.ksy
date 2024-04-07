# Not tested separately, only as imported from
# "imports_params_def_array_usertype_imported".
#
# This spec tests that the `import` statement is inserted into the generated
# code (in languages where it is needed) for the imported .ksy module if we only
# declare a top-level parameter of an array type whose base type is the imported
# top-level type.
meta:
  id: params_def_array_usertype_imported
  imports:
    - hello_world
params:
  - id: hws_param
    type: hello_world[]
instances:
  hw0_one:
    value: hws_param[0].one
  hw1_one:
    value: hws_param[1].one
