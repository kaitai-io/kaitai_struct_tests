local luaunit = require("luaunit")

require("recursive_one")

TestRecursiveOne = {}

function TestRecursiveOne:test_recursive_one()
    local r = RecursiveOne:from_file("src/fixed_struct.bin")

    luaunit.assertEquals(r.one, 0x50)
    luaunit.assertEquals(r.next.one, 0x41)
    luaunit.assertEquals(r.next.next.one, 0x43)
    luaunit.assertEquals(r.next.next.next.finisher, 0x2d4b)
end
