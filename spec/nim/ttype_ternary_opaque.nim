import os, streams, options, sequtils
import ../../compiled/nim/type_ternary_opaque
import auxiliary/test_utils

let r = TypeTernaryOpaque.fromFile("../../src/term_strz.bin")

assert r.dif.one == 102
