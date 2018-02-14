local luaunit = require("luaunit")

require("term_bytes")

TestTermBytes = {}

function TestTermBytes:test_term_bytes()
    local r = TermBytes:from_file("src/term_strz.bin")

    luaunit.assertEquals(r.s1, "foo")
    luaunit.assertEquals(r.s2, "bar")
    luaunit.assertEquals(r.s3, "|baz@")
end
