local luaunit = require("luaunit")

require("opaque_with_param")

TestOpaqueWithParam = {}

function TestOpaqueWithParam:test_opaque_with_param()
    local r = OpaqueWithParam:from_file("src/term_strz.bin")

    luaunit.assertEquals(r.one.buf, "foo|b")
    luaunit.assertEquals(r.one.trailer, 0x61)
end
