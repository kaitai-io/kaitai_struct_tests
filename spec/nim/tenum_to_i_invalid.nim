# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/enum_to_i_invalid
import auxiliary/test_utils

let r = EnumToIInvalid.fromFile("../../src/term_strz.bin")

assert r.pet1 == enum_to_i_invalid.dog
assert r.pet2 == 111
assert r.pet2I == 111
assert r.pet2IToS == "111"
assert r.pet2Mod == 32879
assert r.oneLtTwo == true
assert r.pet2EqIntT == true
assert r.pet2EqIntF == false
