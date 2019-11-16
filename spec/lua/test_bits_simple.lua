local luaunit = require("luaunit")

require("bits_simple")

TestBitsSimple = {}

function TestBitsSimple:test_bits_simple()
    local r = BitsSimple:from_file("src/fixed_struct.bin")

    -- 50 41
    luaunit.assertEquals(r.byte_1, 0x50)
    luaunit.assertEquals(r.byte_2, 0x41)

    -- 43 (1 + 3 + 4) = 0|100|0011
    luaunit.assertEquals(r.bits_a, 0)
    luaunit.assertEquals(r.bits_b, 0x4)
    luaunit.assertEquals(r.bits_c, 0x3)

    -- 4B 2D 31 (10 + 3 + 11) = 01001011 00|101|101 00110001
    luaunit.assertEquals(r.large_bits_1, 0x12c)
    luaunit.assertEquals(r.spacer, 0x5)
    luaunit.assertEquals(r.large_bits_2, 0x531)

    -- FF FF
    luaunit.assertEquals(r.normal_s2, -1)

    -- 50 41 43
    luaunit.assertEquals(r.byte_8_9_10, 0x504143)

    -- 4B 2D 55 2D
    luaunit.assertEquals(r.byte_11_to_14, 0x4B2D552D)

    -- 44 45 46 FF FF
    luaunit.assertEquals(r.byte_15_to_19, 0x444546FFFF)

    -- FF FF FF FF FF FF FF FF
    luaunit.assertEquals(r.byte_20_to_27, 0xFFFFFFFFFFFFFFFF)

    luaunit.assertEquals(r.test_if_b1, 123)
end
