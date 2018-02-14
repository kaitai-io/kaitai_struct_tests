local luaunit = require("luaunit")

require("buffered_struct")

TestBufferedStruct = {}

function TestBufferedStruct:test_buffered_struct()
    local r = BufferedStruct:from_file("src/buffered_struct.bin")

    luaunit.assertEquals(r.len1, 0x10)
    luaunit.assertEquals(r.block1.number1, 0x42)
    luaunit.assertEquals(r.block1.number2, 0x43)
    luaunit.assertEquals(r.len2, 0x8)
    luaunit.assertEquals(r.block2.number1, 0x44)
    luaunit.assertEquals(r.block2.number2, 0x45)
    luaunit.assertEquals(r.finisher, 0xee)
end
