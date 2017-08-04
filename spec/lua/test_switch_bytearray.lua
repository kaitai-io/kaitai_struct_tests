local luaunit = require("luaunit")

require("switch_bytearray")

TestSwitchBytearray = {}

function TestSwitchBytearray:test_switch_bytearray()
    local r = SwitchBytearray:from_file("src/switch_opcodes.bin")

    luaunit.assertEquals(#r.opcodes, 4)

    luaunit.assertEquals(r.opcodes[1].code, "\083")
    luaunit.assertEquals(r.opcodes[1].body.value, "foobar")

    luaunit.assertEquals(r.opcodes[2].code, "\073")
    luaunit.assertEquals(r.opcodes[2].body.value, 0x42)

    luaunit.assertEquals(r.opcodes[3].code, "\073")
    luaunit.assertEquals(r.opcodes[3].body.value, 0x37)

    luaunit.assertEquals(r.opcodes[4].code, "\083")
    luaunit.assertEquals(r.opcodes[4].body.value, "")
end
