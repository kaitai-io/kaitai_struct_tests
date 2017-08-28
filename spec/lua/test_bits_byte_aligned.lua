local luaunit = require("luaunit")

require("bits_byte_aligned")

TestBitsByteAligned = {}

function TestBitsByteAligned:test_bits_byte_aligned()
    local r = BitsByteAligned:from_file("src/fixed_struct.bin")

    -- 50 (6 + 2) = 010100|00
    luaunit.assertEquals(r.one, 0x14)
    -- 41
    luaunit.assertEquals(r.byte_1, 0x41)
    -- 43 (3 + 1 + 4) = 010|0|0011
    luaunit.assertEquals(r.two, 0x02)
    luaunit.assertEquals(r.three, 0)
    -- 4B
    luaunit.assertEquals(r.byte_2, 0x4b)
    -- 2D 31 (14 + 2) = 00101101 001100|01
    luaunit.assertEquals(r.four, 0xb4c)
    -- FF
    luaunit.assertEquals(r.byte_3, "\255")
    -- FF
    luaunit.assertEquals(r.full_byte, 0xff)
    -- 50
    luaunit.assertEquals(r.byte_4, 0x50)
end
