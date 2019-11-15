local luaunit = require("luaunit")

require("type_ternary")

TestTypeTernary = {}

function TestTypeTernary:test_type_ternary()
    local r = TypeTernary:from_file("src/term_strz.bin")

    luaunit.assertEquals(r.dif.value, 0x65)
end
