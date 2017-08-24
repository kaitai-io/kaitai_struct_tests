local luaunit = require("luaunit")

require("switch_manual_int_else")

TestSwitchManualIntElse = {}

function TestSwitchManualIntElse:test_switch_manual_int_else()
    local r = SwitchManualIntElse:from_file("src/switch_opcodes2.bin")

    luaunit.assertEquals(#r.opcodes, 4)

    luaunit.assertEquals(r.opcodes[1].code, 83)
    luaunit.assertEquals(r.opcodes[1].body.value, "foo")

    luaunit.assertEquals(r.opcodes[2].code, 88)
    luaunit.assertEquals(r.opcodes[2].body.filler, 0x42)

    luaunit.assertEquals(r.opcodes[3].code, 89)
    luaunit.assertEquals(r.opcodes[3].body.filler, 0xcafe)

    luaunit.assertEquals(r.opcodes[4].code, 73)
    luaunit.assertEquals(r.opcodes[4].body.value, 7)
end
