# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/expr_0
import auxiliary/test_utils

let r = Expr0.fromFile("../../src/str_encodings.bin")

assert r.mustBeF7 == 247
assert r.mustBeAbc123 == "abc123"
