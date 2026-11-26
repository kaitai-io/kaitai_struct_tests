meta:
  id: reserved_python_keywords
seq:
  # Python keywords as field names
  # Ref: https://docs.python.org/3/reference/lexical_analysis.html#keywords
  - id: and
    type: u1
  - id: as
    type: u1
  - id: assert
    type: u1
  - id: async
    type: u1
  - id: await
    type: u1
  - id: break
    type: u1
  - id: class
    type: u1
  - id: continue
    type: u1
  - id: def
    type: u1
  - id: del
    type: u1
  - id: elif
    type: u1
  - id: else
    type: u1
  - id: except
    type: u1
  - id: 'false' # False
    type: u1
  - id: finally
    type: u1
  - id: for
    type: u1
  - id: from
    type: u1
  - id: global
    type: u1
  - id: if
    type: u1
  - id: import
    type: u1
  - id: in
    type: u1
  - id: is
    type: u1
  - id: lambda
    type: u1
  - id: none # None
    type: u1
  - id: nonlocal
    type: u1
  - id: not
    type: u1
  - id: or
    type: u1
  - id: pass
    type: u1
  - id: raise
    type: u1
  - id: return
    type: u1
  - id: 'true' # True
    type: u1
  - id: try
    type: u1
  - id: while
    type: u1
  - id: with
    type: u1
  - id: yield
    type: u1
  # Test user types with keyword names
  - id: keyword_struct_def
    type: def
  - id: keyword_struct_false
    type: 'false'
  # Test enums with keyword names
  - id: keyword_enum
    type: u1
    enum: 'true'
  - id: keyword_nested_enum
    type: u1
    enum: 'def::try'
instances:
  # Value instance using keyword-named seq attributes
  inst_keyword_seq:
    value: and + as
  # Value instance using keyword-named enum and keyword-named enum values
  inst_keyword_enum:
    value: true::if
  # Value instance using keyword-named enum and keyword-named enum values, converted to integer
  inst_keyword_enum_to_i:
    value: true::if.to_i
types:
  # Type with keyword name containing seq atttributes with keyword names
  def:
    seq:
      - id: while
        type: u1
      - id: for
        type: u1
      - id: with
        type: u1
    instances:
      # Instance with keyword name in keyword-named type
      return:
        value: while + for
    enums:
      # Enum with keyword name in keyword-named type
      try:
        1: nonlocal
        2: finally
        3: raise
        40: except
  # Default capitalization of class names will result in "False" as class name without escaping
  false:
    seq:
      - id: pass
        type: u1
enums:
  # Enum with keyword name, containing keyword-named values
  'true':
    0: 'false'
    1: 'true'
    2: none
    4: or
    5: not
    6: if
    7: else
    39: and
