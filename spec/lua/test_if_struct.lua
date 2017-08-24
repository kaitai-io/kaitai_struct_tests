local luaunit = require("luaunit")

require("if_struct")

TestIfStruct = {}

function TestIfStruct:test_if_struct()
    local r = IfStruct:from_file("src/if_struct.bin")

    luaunit.assertEquals(r.op1.opcode, 0x53)
    luaunit.assertEquals(r.op1.arg_str.str, "foo")

    luaunit.assertEquals(r.op2.opcode, 0x54)
    luaunit.assertEquals(r.op2.arg_tuple.num1, 0x42)
    luaunit.assertEquals(r.op2.arg_tuple.num2, 0x43)

    luaunit.assertEquals(r.op3.opcode, 0x53)
    luaunit.assertEquals(r.op3.arg_str.str, "bar")
end
