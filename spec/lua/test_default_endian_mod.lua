local luaunit = require("luaunit")

require("default_endian_mod")

TestDefaultEndianMod = {}

function TestDefaultEndianMod:test_default_endian_mod()
    local r = DefaultEndianMod:from_file("src/fixed_struct.bin")

    luaunit.assertEquals(r.main.one, 0x4b434150)
    luaunit.assertEquals(r.main.nest.two, -52947)
    luaunit.assertEquals(r.main.nest_be.two, 0x5041434b)
end
