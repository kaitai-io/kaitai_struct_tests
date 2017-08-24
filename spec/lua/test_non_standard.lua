local luaunit = require("luaunit")

require("non_standard")

TestNonStandard = {}

function TestNonStandard:test_non_standard()
    local r = NonStandard:from_file("src/fixed_struct.bin")

    luaunit.assertEquals(r.foo, 0x50)
end
