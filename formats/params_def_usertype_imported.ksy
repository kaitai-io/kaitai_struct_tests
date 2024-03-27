# Not tested separately, only as imported from
# "imports_params_def_usertype_imported".
#
# This spec tests that the `import` statement is inserted into the generated
# code (in languages where it is needed) for the imported .ksy module if it's
# only used as a top-level parameter type.
meta:
  id: params_def_usertype_imported
  imports:
    - hello_world
params:
  - id: hw_param
    type: hello_world
instances:
  hw_one:
    value: hw_param.one
