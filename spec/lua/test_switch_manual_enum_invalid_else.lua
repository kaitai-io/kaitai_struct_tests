local luaunit = require("luaunit")

require("switch_manual_enum_invalid_else")

TestSwitchManualEnumInvalidElse = {}

function TestSwitchManualEnumInvalidElse:test_switch_manual_enum_invalid_else()
    local r = SwitchManualEnumInvalidElse:from_file("src/enum_negative.bin")

    luaunit.assertEquals(#r.opcodes, 2)
    luaunit.assertNil(r.opcodes[0 + 1].code)
    luaunit.assertEquals(r.opcodes[0 + 1].body.value, 123)
    luaunit.assertNil(r.opcodes[1 + 1].code)
    luaunit.assertEquals(r.opcodes[1 + 1].body.value, 123)
end
