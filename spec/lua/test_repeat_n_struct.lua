local luaunit = require("luaunit")

require("repeat_n_struct")

TestRepeatNStruct = {}

function TestRepeatNStruct:test_repeat_n_struct()
    local r = RepeatNStruct:from_file("src/repeat_n_struct.bin")

    luaunit.assertEquals(#r.chunks, 2)
    luaunit.assertEquals(r.chunks[1].offset, 0x10)
    luaunit.assertEquals(r.chunks[1].len, 0x2078)
    luaunit.assertEquals(r.chunks[2].offset, 0x2088)
    luaunit.assertEquals(r.chunks[2].len, 0xf)
end
