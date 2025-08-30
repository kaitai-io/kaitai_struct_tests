import os, streams, options, sequtils
import ../../compiled/nim/opaque_external_type
import auxiliary/test_utils

let r = OpaqueExternalType.fromFile("../../src/term_strz.bin")

assert r.hw.one == 102
