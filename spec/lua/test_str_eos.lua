local luaunit = require("luaunit")

require("str_eos")

TestStrEos = {}

function TestStrEos:test_str_eos()
    local r = StrEos:from_file("src/term_strz.bin")

    luaunit.assertEquals(r.str, "foo|bar|baz@")
end
