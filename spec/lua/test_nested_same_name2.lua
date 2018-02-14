local luaunit = require("luaunit")

require("nested_same_name2")

TestNestedSameName2 = {}

function TestNestedSameName2:test_nested_same_name2()
    local r = NestedSameName2:from_file("src/nested_same_name2.bin")

    luaunit.assertEquals(r.version, 0x42)
    luaunit.assertEquals(r.main_data.main_size, 2)
    luaunit.assertEquals(r.main_data.foo.data1, "\x11\x11\x11\x11")
    luaunit.assertEquals(r.dummy.dummy_size, 3)
    luaunit.assertEquals(r.dummy.foo.data2, "\x22\x22\x22\x22\x22\x22")
end
