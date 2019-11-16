local luaunit = require("luaunit")

require("enum_if")

TestEnumIf = {}

function TestEnumIf:test_enum_if()
    local r = EnumIf:from_file("src/if_struct.bin")

    luaunit.assertEquals(r.op1.opcode, EnumIf.Opcodes.a_string)
    luaunit.assertEquals(r.op1.arg_str.str, "foo")

    luaunit.assertEquals(r.op2.opcode, EnumIf.Opcodes.a_tuple)
    luaunit.assertEquals(r.op2.arg_tuple.num1, 0x42)
    luaunit.assertEquals(r.op2.arg_tuple.num2, 0x43)

    luaunit.assertEquals(r.op3.opcode, EnumIf.Opcodes.a_string)
    luaunit.assertEquals(r.op3.arg_str.str, "bar")
end
