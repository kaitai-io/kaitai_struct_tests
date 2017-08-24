local luaunit = require("luaunit")

require("bits_enum")

TestBitsEnum = {}

function TestBitsEnum:test_bits_enum()
    local r = BitsEnum:from_file("src/fixed_struct.bin")

    -- 50 41 (4 + 8 + 1) = 0101|0000 0100|0|001
    luaunit.assertEquals(r.one, BitsEnum.Animal.platypus)
    luaunit.assertEquals(r.two, BitsEnum.Animal.horse)
    luaunit.assertEquals(r.three, BitsEnum.Animal.cat)
end
