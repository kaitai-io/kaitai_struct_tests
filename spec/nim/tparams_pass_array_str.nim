# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/params_pass_array_str
import auxiliary/test_utils

let r = ParamsPassArrayStr.fromFile("../../src/term_strz.bin")

assert len(r.passStrArray.strs) == 3
assert r.passStrArray.strs[0] == "fo"
assert r.passStrArray.strs[1] == "o|"
assert r.passStrArray.strs[2] == "ba"
assert len(r.passStrArrayCalc.strs) == 2
assert r.passStrArrayCalc.strs[0] == "aB"
assert r.passStrArrayCalc.strs[1] == "Cd"
