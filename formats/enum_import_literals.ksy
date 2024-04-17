# Tests whether the `import` statement is inserted into the generated code (in
# languages where it's needed) when referencing an enum of an imported .ksy file
# using enum literals, and whether references to external enums are generated
# correctly.
meta:
  id: enum_import_literals
  imports:
    - enum_0
    - enum_deep
instances:
  pet_1_to_i:
    value: enum_0::animal::cat.to_i
  pet_1_eq:
    value: '(true ? enum_0::animal::chicken : enum_0::animal::dog) == enum_0::animal::chicken'
  pet_2:
    value: enum_deep::container1::container2::animal::hare
