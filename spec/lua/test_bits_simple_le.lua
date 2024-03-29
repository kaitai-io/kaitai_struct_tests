-- Autogenerated from KST: please remove this line if doing any edits by hand!

local luaunit = require("luaunit")

require("bits_simple_le")

TestBitsSimpleLe = {}

function TestBitsSimpleLe:test_bits_simple_le()
    local r = BitsSimpleLe:from_file("src/fixed_struct.bin")

    luaunit.assertEquals(r.byte_1, 80)
    luaunit.assertEquals(r.byte_2, 65)
    luaunit.assertEquals(r.bits_a, true)
    luaunit.assertEquals(r.bits_b, 1)
    luaunit.assertEquals(r.bits_c, 4)
    luaunit.assertEquals(r.large_bits_1, 331)
    luaunit.assertEquals(r.spacer, 3)
    luaunit.assertEquals(r.large_bits_2, 393)
    luaunit.assertEquals(r.normal_s2, -1)
    luaunit.assertEquals(r.byte_8_9_10, 4407632)
    luaunit.assertEquals(r.byte_11_to_14, 760556875)
    luaunit.assertEquals(r.byte_15_to_19, 1099499455812)
    luaunit.assertEquals(r.byte_20_to_27, 0xffffffffffffffff)
    luaunit.assertEquals(r.test_if_b1, 123)
end
