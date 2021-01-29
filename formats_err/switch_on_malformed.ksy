# switch_on_malformed.ksy: /seq/0/switch-on: error: parsing expression '42/' failed on 1:3, expected "or" | CharsWhile(Set( , n)) | "\\\n" | End
meta:
  id: switch_on_malformed
seq:
  - id: foo
    type:
      switch-on: 42/
      cases:
        42: really
types:
  really: {}
