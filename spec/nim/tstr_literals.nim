import os, streams, options, sequtils, unicode
import ../../compiled/nim/str_literals
import auxiliary/test_utils

let r = StrLiterals.fromFile("../../src/fixed_struct.bin")

proc strToArr(s: string): seq[int32] = toRunes(s).map(proc(x: Rune): int32 = int32(x))

assert strToArr(r.complexStr) == @[0'i32, 1, 2, 7, 8, 10, 13, 9, 11, 12, 27, 61, 7, 10, 36, 9787]
assert strToArr(r.doubleQuotes) == @[34'i32, 34, 34]
assert strToArr(r.backslashes) == @[92'i32, 92, 92]
assert strToArr(r.octalEatup) == @[0'i32, 50, 50]
assert strToArr(r.octalEatup2) == @[2'i32, 50]
