local luaunit = require("luaunit")

require("switch_integers2")

TestSwitchIntegers2 = {}

function TestSwitchIntegers2:test_switch_integers2()
    local r = SwitchIntegers2:from_file("src/switch_integers.bin")

    luaunit.assertEquals(r.code, 1)
    luaunit.assertEquals(r.len, 7)
    luaunit.assertEquals(r.ham, "\002\064\064\004\055\019\000")
    luaunit.assertEquals(r.padding, 0)
    luaunit.assertEquals(r.len_mod_str, "13")
end
