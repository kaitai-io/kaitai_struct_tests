-- Autogenerated from KST: please remove this line if doing any edits by hand!

local luaunit = require("luaunit")

require("bits_signed_shift_b64_le")

TestBitsSignedShiftB64Le = {}

function TestBitsSignedShiftB64Le:test_bits_signed_shift_b64_le()
    local r = BitsSignedShiftB64Le:from_file("src/bits_signed_shift_b64_le.bin")

    luaunit.assertEquals(r.a, 0)
    luaunit.assertEquals(r.b, 255)
end
