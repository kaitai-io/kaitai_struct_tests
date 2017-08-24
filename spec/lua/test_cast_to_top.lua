local luaunit = require("luaunit")

require("cast_to_top")

TestCastToTop = {}

function TestCastToTop:test_cast_to_top()
    local r = CastToTop:from_file("src/fixed_struct.bin")

    luaunit.assertEquals(r.code, 0x50)
    luaunit.assertEquals(r.header.code, 0x41)
    luaunit.assertEquals(r.header_casted.code, 0x41)
end
