-- Autogenerated from KST: please remove this line if doing any edits by hand!

local luaunit = require("luaunit")

require("nested_types3")

TestNestedTypes3 = {}

function TestNestedTypes3:test_nested_types3()
    local r = NestedTypes3:from_file("src/fixed_struct.bin")

    luaunit.assertEquals(r.a_cc.value_cc, 80)
    luaunit.assertEquals(r.a_c_d.value_d, 65)
    luaunit.assertEquals(r.b.value_b, 67)
    luaunit.assertEquals(r.b.a_cc.value_cc, 75)
    luaunit.assertEquals(r.b.a_c_d.value_d, 45)
end
