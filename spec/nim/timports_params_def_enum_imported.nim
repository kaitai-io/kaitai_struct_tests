# Autogenerated from KST: please remove this line if doing any edits by hand!

import ../../compiled/nim/enum_0
import ../../compiled/nim/enum_deep
import os, streams, options, sequtils
import ../../compiled/nim/imports_params_def_enum_imported
import auxiliary/test_utils

let r = ImportsParamsDefEnumImported.fromFile("../../src/enum_0.bin")

assert r.one.pet1 == enum_0.cat
assert r.one.pet2 == enum_deep.hare
assert r.two.pet1Param == enum_0.cat
assert r.two.pet2Param == enum_deep.hare