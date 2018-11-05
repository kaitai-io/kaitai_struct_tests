# /instances/both: parsing expression 'foo == 1 && bar == 2' failed on 1:10, expected "or" | CharsWhile(Set( , n)) | "\\\n" | End, did you mean 'and'?
meta:
  id: expr_wrong_and
seq:
  - id: foo
    type: u1
  - id: bar
    type: u1
instances:
  both:
    value: foo == 1 && bar == 2
