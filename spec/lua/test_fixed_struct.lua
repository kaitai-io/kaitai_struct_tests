-- Autogenerated from KST: please remove this line if doing any edits by hand!

local luaunit = require("luaunit")

require("fixed_struct")

TestFixedStruct = {}

function TestFixedStruct:test_fixed_struct()
    local r = FixedStruct:from_file("src/fixed_struct.bin")

    luaunit.assertEquals(r.hdr.uint8, 255)
    luaunit.assertEquals(r.hdr.uint16, 65535)
    luaunit.assertEquals(r.hdr.uint32, 4294967295)
    luaunit.assertEquals(r.hdr.uint64, 0xffffffffffffffff)
    luaunit.assertEquals(r.hdr.sint8, -1)
    luaunit.assertEquals(r.hdr.sint16, -1)
    luaunit.assertEquals(r.hdr.sint32, -1)
    luaunit.assertEquals(r.hdr.sint64, -1)
    luaunit.assertEquals(r.hdr.uint16le, 66)
    luaunit.assertEquals(r.hdr.uint32le, 66)
    luaunit.assertEquals(r.hdr.uint64le, 66)
    luaunit.assertEquals(r.hdr.sint16le, -66)
    luaunit.assertEquals(r.hdr.sint32le, -66)
    luaunit.assertEquals(r.hdr.sint64le, -66)
    luaunit.assertEquals(r.hdr.uint16be, 66)
    luaunit.assertEquals(r.hdr.uint32be, 66)
    luaunit.assertEquals(r.hdr.uint64be, 66)
    luaunit.assertEquals(r.hdr.sint16be, -66)
    luaunit.assertEquals(r.hdr.sint32be, -66)
    luaunit.assertEquals(r.hdr.sint64be, -66)
end
