-- Autogenerated from KST: please remove this line if doing any edits by hand!

local luaunit = require("luaunit")

require("switch_manual_enum_invalid")

TestSwitchManualEnumInvalid = {}

function TestSwitchManualEnumInvalid:test_switch_manual_enum_invalid()
    local r = SwitchManualEnumInvalid:from_file("src/enum_negative.bin")

    luaunit.assertEquals(#r.opcodes, 2)
    luaunit.assertEquals(r.opcodes[0 + 1].code, 255)
    luaunit.assertNil(r.opcodes[0 + 1].body)
    luaunit.assertEquals(r.opcodes[1 + 1].code, 1)
    luaunit.assertNil(r.opcodes[1 + 1].body)
end
