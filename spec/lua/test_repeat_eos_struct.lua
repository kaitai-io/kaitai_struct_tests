local luaunit = require("luaunit")

require("repeat_eos_struct")

TestRepeatEosStruct = {}

function TestRepeatEosStruct:test_repeat_eos_struct()
    local r = RepeatEosStruct:from_file("src/repeat_eos_struct.bin")

    luaunit.assertEquals(#r.chunks, 2)
    luaunit.assertEquals(r.chunks[1].offset, 0)
    luaunit.assertEquals(r.chunks[1].len, 0x42)
    luaunit.assertEquals(r.chunks[2].offset, 0x42)
    luaunit.assertEquals(r.chunks[2].len, 0x815)
end
