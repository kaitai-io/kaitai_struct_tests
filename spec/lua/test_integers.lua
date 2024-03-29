-- Autogenerated from KST: please remove this line if doing any edits by hand!

local luaunit = require("luaunit")

require("integers")

TestIntegers = {}

function TestIntegers:test_integers()
    local r = Integers:from_file("src/fixed_struct.bin")

    luaunit.assertEquals(r.uint8, 255)
    luaunit.assertEquals(r.uint16, 65535)
    luaunit.assertEquals(r.uint32, 4294967295)
    luaunit.assertEquals(r.uint64, 0xffffffffffffffff)
    luaunit.assertEquals(r.sint8, -1)
    luaunit.assertEquals(r.sint16, -1)
    luaunit.assertEquals(r.sint32, -1)
    luaunit.assertEquals(r.sint64, -1)
    luaunit.assertEquals(r.uint16le, 66)
    luaunit.assertEquals(r.uint32le, 66)
    luaunit.assertEquals(r.uint64le, 66)
    luaunit.assertEquals(r.sint16le, -66)
    luaunit.assertEquals(r.sint32le, -66)
    luaunit.assertEquals(r.sint64le, -66)
    luaunit.assertEquals(r.uint16be, 66)
    luaunit.assertEquals(r.uint32be, 66)
    luaunit.assertEquals(r.uint64be, 66)
    luaunit.assertEquals(r.sint16be, -66)
    luaunit.assertEquals(r.sint32be, -66)
    luaunit.assertEquals(r.sint64be, -66)
end
