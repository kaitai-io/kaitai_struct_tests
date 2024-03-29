# Autogenerated from KST: please remove this line if doing any edits by hand!

import os, streams, options, sequtils
import ../../compiled/nim/nav_parent3
import auxiliary/test_utils

let r = NavParent3.fromFile("../../src/nav_parent2.bin")

assert r.ofsTags == 8
assert r.numTags == 2
assert r.tags[0].name == "RAHC"
assert r.tags[0].ofs == 32
assert r.tags[0].numItems == 3
assert r.tags[0].tagContent.content == "foo"
assert r.tags[1].name == "RAHC"
assert r.tags[1].ofs == 35
assert r.tags[1].numItems == 6
assert r.tags[1].tagContent.content == "barbaz"
