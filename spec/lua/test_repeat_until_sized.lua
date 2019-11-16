local luaunit = require("luaunit")

require("repeat_until_sized")

TestRepeatUntilSized = {}

function TestRepeatUntilSized:test_repeat_until_sized()
    local r = RepeatUntilSized:from_file("src/repeat_until_process.bin")

    luaunit.assertEquals(#r.records, 3)

    luaunit.assertEquals(r.records[1].marker, 0xe8)
    luaunit.assertEquals(r.records[1].body, 0xaaaaaaba)

    luaunit.assertEquals(r.records[2].marker, 0xfa)
    luaunit.assertEquals(r.records[2].body, 0xaaaab89e)

    luaunit.assertEquals(r.records[3].marker, 0xaa)
    luaunit.assertEquals(r.records[3].body, 0x55555555)
end
