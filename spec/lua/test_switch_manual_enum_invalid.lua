local luaunit = require("luaunit")

require("switch_manual_enum_invalid")

TestSwitchManualEnumInvalid = {}

function TestSwitchManualEnumInvalid:test_switch_manual_enum_invalid()
    local r = SwitchManualEnumInvalid:from_file("src/enum_negative.bin")

    luaunit.assertEquals(#r.opcodes, 2)
    luaunit.assertNil(r.opcodes[0 + 1].code)
    luaunit.assertNil(r.opcodes[0 + 1].body)
    luaunit.assertNil(r.opcodes[1 + 1].code)
    luaunit.assertNil(r.opcodes[1 + 1].body)
end
