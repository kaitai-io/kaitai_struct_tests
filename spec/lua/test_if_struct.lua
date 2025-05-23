-- Autogenerated from KST: please remove this line if doing any edits by hand!

local luaunit = require("luaunit")

require("if_struct")

TestIfStruct = {}

function TestIfStruct:test_if_struct()
    local r = IfStruct:from_file("src/if_struct.bin")
    luaunit.assertEquals(r.op1.opcode, 83)
    luaunit.assertNil(r.op1.arg_tuple)
    luaunit.assertEquals(r.op1.arg_str.str, "foo")
    luaunit.assertEquals(r.op2.opcode, 84)
    luaunit.assertEquals(r.op2.arg_tuple.num1, 66)
    luaunit.assertEquals(r.op2.arg_tuple.num2, 67)
    luaunit.assertNil(r.op2.arg_str)
    luaunit.assertEquals(r.op3.opcode, 83)
    luaunit.assertNil(r.op3.arg_tuple)
    luaunit.assertEquals(r.op3.arg_str.str, "bar")
end
