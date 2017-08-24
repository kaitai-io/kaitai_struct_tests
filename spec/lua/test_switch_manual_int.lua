local luaunit = require("luaunit")

require("switch_manual_int")

TestSwitchManualInt = {}

function TestSwitchManualInt:test_switch_manual_int()
    local r = SwitchManualInt:from_file("src/switch_opcodes.bin")

    luaunit.assertEquals(#r.opcodes, 4)

    luaunit.assertEquals(r.opcodes[1].code, 83)
    luaunit.assertEquals(r.opcodes[1].body.value, "foobar")

    luaunit.assertEquals(r.opcodes[2].code, 73)
    luaunit.assertEquals(r.opcodes[2].body.value, 0x42)

    luaunit.assertEquals(r.opcodes[3].code, 73)
    luaunit.assertEquals(r.opcodes[3].body.value, 0x37)

    luaunit.assertEquals(r.opcodes[4].code, 83)
    luaunit.assertEquals(r.opcodes[4].body.value, "")
end
