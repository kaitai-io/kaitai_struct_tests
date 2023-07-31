local luaunit = require("luaunit")

require("to_string_custom")

TestToStringCustom = {}

function TestToStringCustom:test_term_strz()
    local r = ToStringCustom:from_file("src/term_strz.bin")

    luaunit.assertEquals(tostring(r), "s1 = foo, s2 = bar")
end
