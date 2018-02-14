local luaunit = require("luaunit")

require("expr_bytes_cmp")

TestExprBytesCmp = {}

function TestExprBytesCmp:test_expr_bytes_cmp()
    local r = ExprBytesCmp:from_file("src/fixed_struct.bin")

    luaunit.assertEquals(r.one, "P")
    luaunit.assertEquals(r.two, "ACK")

    luaunit.assertEquals(r.is_eq, true)
    luaunit.assertEquals(r.is_ne, false)
    luaunit.assertEquals(r.is_lt, true)
    luaunit.assertEquals(r.is_gt, false)
    luaunit.assertEquals(r.is_le, true)
    luaunit.assertEquals(r.is_ge, false)
    luaunit.assertEquals(r.is_lt2, false)
    luaunit.assertEquals(r.is_gt2, true)
end
