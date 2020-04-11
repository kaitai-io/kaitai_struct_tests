import os, streams, options, sequtils, unittest
import ../../../compiled/nim/opaque_external_type_02_parent
import ../test_utils

let r = OpaqueExternalType02Parent.fromFile("src/term_strz.bin")

test "OpaqueExternalType02Parent":

  check(r.parent.child.s1 == string("foo"))
  check(r.parent.child.s2 == string("bar"))
  check(r.parent.child.s3.s3 == string("|baz@"))
  discard
