local luaunit = require("luaunit")

require("repeat_eos_u4")

TestRepeatEosU4 = {}

function TestRepeatEosU4:test_repeat_eos_u4()
    local r = RepeatEosU4:from_file("src/repeat_eos_struct.bin")

    luaunit.assertEquals(r.numbers, {0, 0x42, 0x42, 0x815})
end
