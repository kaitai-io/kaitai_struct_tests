local luaunit = require("luaunit")

require("type_ternary_opaque")

TestTypeTernaryOpaque = {}

function TestTypeTernaryOpaque:test_type_ternary_opaque()
    local r = TypeTernaryOpaque:from_file("src/term_strz.bin")

    luaunit.assertEquals(r.dif.s1, "foo")
    luaunit.assertEquals(r.dif.s2, "bar")
    luaunit.assertEquals(r.dif.s3, "|baz@")
end
