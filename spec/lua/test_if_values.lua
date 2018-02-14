local luaunit = require("luaunit")

require("if_values")

TestIfValues = {}

function TestIfValues:test_if_values()
    local r = IfValues:from_file("src/fixed_struct.bin")

    luaunit.assertEquals(r.codes[1].opcode, 80)
    luaunit.assertEquals(r.codes[1].half_opcode, 40)
    luaunit.assertEquals(r.codes[2].opcode, 65)
    luaunit.assertNil(r.codes[2].half_opcode)
    luaunit.assertEquals(r.codes[3].opcode, 67)
    luaunit.assertNil(r.codes[3].half_opcode)
end
