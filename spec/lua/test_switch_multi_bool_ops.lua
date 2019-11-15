local luaunit = require("luaunit")

require("switch_multi_bool_ops")

TestSwitchMultiBoolOps = {}

function TestSwitchMultiBoolOps:test_switch_multi_bool_ops()
    local r = SwitchMultiBoolOps:from_file("src/switch_integers.bin")

    luaunit.assertEquals(#r.opcodes, 4)

    luaunit.assertEquals(r.opcodes[1].code, 1)
    luaunit.assertEquals(r.opcodes[1].body, 7)

    luaunit.assertEquals(r.opcodes[2].code, 2)
    luaunit.assertEquals(r.opcodes[2].body, 0x4040)

    luaunit.assertEquals(r.opcodes[3].code, 4)
    luaunit.assertEquals(r.opcodes[3].body, 4919)

    luaunit.assertEquals(r.opcodes[4].code, 8)
    luaunit.assertEquals(r.opcodes[4].body, 4919)
end
