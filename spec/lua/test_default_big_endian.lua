local luaunit = require("luaunit")

require("default_big_endian")

TestDefaultBigEndian = {}

function TestDefaultBigEndian:test_default_big_endian()
    local r = DefaultBigEndian:from_file("src/enum_0.bin")

    luaunit.assertEquals(r.one, 0x7000000)
end
