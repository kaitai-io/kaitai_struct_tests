# Not tested separately, only as imported from
# "imports_params_def_enum_imported".
#
# This spec tests that the `import` statement is inserted into the generated
# code (in languages where it is needed) for the imported .ksy module if it's
# only used as a source of the enum used in the top-level parameter type
# declaration.
meta:
  id: params_def_enum_imported
  imports:
    - enum_0
    - enum_deep
params:
  - id: pet_1_param
    type: u4
    enum: enum_0::animal
  - id: pet_2_param
    type: u4
    enum: enum_deep::container1::container2::animal
