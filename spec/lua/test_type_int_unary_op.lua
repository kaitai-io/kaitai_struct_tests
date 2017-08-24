local luaunit = require("luaunit")

require("type_int_unary_op")

TestTypeIntUnaryOp = {}

function TestTypeIntUnaryOp:test_type_int_unary_op()
    local r = TypeIntUnaryOp:from_file("src/fixed_struct.bin")

    luaunit.assertEquals(r.value_s2, 0x4150)
    luaunit.assertEquals(r.value_s8, 0x4150ffff312d4b43)
    luaunit.assertEquals(r.unary_s2, -0x4150)
    luaunit.assertEquals(r.unary_s8, -0x4150ffff312d4b43)
end
