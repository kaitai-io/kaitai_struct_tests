local luaunit = require("luaunit")

require("imports0")

TestImports0 = {}

function TestImports0:test_imports0()
    local r = Imports0:from_file("src/fixed_struct.bin")

    luaunit.assertEquals(r.two, 0x50)
    luaunit.assertEquals(r.hw.one, 0x41)
    luaunit.assertEquals(r.hw_one, 0x41)
end
