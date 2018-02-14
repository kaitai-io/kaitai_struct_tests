local luaunit = require("luaunit")

require("switch_manual_str_else")

TestSwitchManualStrElse = {}

function TestSwitchManualStrElse:test_switch_manual_str_else()
    local r = SwitchManualStrElse:from_file("src/switch_opcodes2.bin")

    luaunit.assertEquals(#r.opcodes, 4)

    luaunit.assertEquals(r.opcodes[1].code, "S")
    luaunit.assertEquals(r.opcodes[1].body.value, "foo")

    luaunit.assertEquals(r.opcodes[2].code, "X")
    luaunit.assertEquals(r.opcodes[2].body.filler, 0x42)

    luaunit.assertEquals(r.opcodes[3].code, "Y")
    luaunit.assertEquals(r.opcodes[3].body.filler, 0xcafe)

    luaunit.assertEquals(r.opcodes[4].code, "I")
    luaunit.assertEquals(r.opcodes[4].body.value, 7)
end
