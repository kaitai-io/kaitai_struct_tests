local luaunit = require("luaunit")

require("opaque_external_type")

TestOpaqueExternalType = {}

function TestOpaqueExternalType:test_opaque_external_type()
    local r = OpaqueExternalType:from_file("src/term_strz.bin")

    luaunit.assertEquals(r.one.s1, "foo")
    luaunit.assertEquals(r.one.s2, "bar")
    luaunit.assertEquals(r.one.s3, "|baz@")
end
