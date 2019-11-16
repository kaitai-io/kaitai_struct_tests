local luaunit = require("luaunit")

require("multiple_use")

TestMultipleUse = {}

function TestMultipleUse:test_multiple_use()
    local r = MultipleUse:from_file("src/position_abs.bin")

    luaunit.assertEquals(r.t1.first_use.value, 0x20)
    luaunit.assertEquals(r.t2.second_use.value, 0x20)
end
