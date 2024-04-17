# Not tested separately, only as imported from "imports_cast_to_imported".
#
# This test generally covers two things:
#
# 1. The type cast `.as<>` operation can resolve the imported type.
# 2. In statically typed languages where a type casting operation needs to be
#    translated (as opposed to dynamically typed languages, where type casting
#    is no-op) and at the same time imports of other .ksy modules must be
#    explicitly specified, merely the occurrence of the imported `hello_world`
#    type in the translated `.as<hello_world>` operation is enough to include
#    the `import` statement.
#
#    To ensure that the test covers this case, the following implementation is
#    careful to keep `.as<hello_world>` the only use of `hello_world`. In that
#    effort, the `hello_world` type was also avoided as the type of the `hw_one`
#    value instance, fearing that the compiler might add the `import` just
#    because it is the resulting type of `hw_one`, not because of the type
#    casting.
meta:
  id: cast_to_imported
  imports:
    - hello_world
params:
  - id: hw_param
    type: struct
instances:
  hw_one:
    value: hw_param.as<hello_world>.one
