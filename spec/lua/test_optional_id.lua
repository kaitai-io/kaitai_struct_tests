local luaunit = require("luaunit")

require("optional_id")

TestOptionalId = {}

function TestOptionalId:test_optional_id()
    local r = OptionalId:from_file("src/fixed_struct.bin")

    luaunit.assertEquals(r._unnamed0, 0x50)
    luaunit.assertEquals(r._unnamed1, 0x41)
    luaunit.assertEquals(r._unnamed2, "\x43\x4b\x2d\x31\xff")
end
