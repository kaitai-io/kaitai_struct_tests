# Not tested separately, only as imported from "imports_cast_to_imported2".
#
# Like "cast_to_imported", but in addition to using the imported type
# `hello_world` in type casting, it is also the resulting type of the `hw` value
# instance.
#
# The idea is that in dynamically typed languages with type hints or type
# annotations, type casting can still be no-op (so there's no need to import the
# external type just because it is used in type casting), but importing the
# external type might be necessary for the type annotation of `hw`.
meta:
  id: cast_to_imported2
  imports:
    - hello_world
params:
  - id: hw_param
    type: struct
instances:
  hw:
    value: hw_param.as<hello_world>
