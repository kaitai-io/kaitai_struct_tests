# Autogenerated from KST: please remove this line if doing any edits by hand!

let r = Integers.fromFile("src" / "fixed_struct.bin")

check(r.uint8 == 255)
check(r.uint16 == 65535)
check(r.uint32 == 4294967295'u32)
check(r.uint64 == 18446744073709551615'u64)
check(r.sint8 == -1)
check(r.sint16 == -1)
check(r.sint32 == -1)
check(r.sint64 == -1)
check(r.uint16le == 66)
check(r.uint32le == 66)
check(r.uint64le == 66)
check(r.sint16le == -66)
check(r.sint32le == -66)
check(r.sint64le == -66)
check(r.uint16be == 66)
check(r.uint32be == 66)
check(r.uint64be == 66)
check(r.sint16be == -66)
check(r.sint32be == -66)
check(r.sint64be == -66)