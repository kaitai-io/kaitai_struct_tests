local luaunit = require("luaunit")

require("expr_0")

TestExpr0 = {}

function TestExpr0:test_expr_0()
    local r = Expr0:from_file("src/str_encodings.bin")

    luaunit.assertEquals(r.must_be_f7, 0xf7)
    luaunit.assertEquals(r.must_be_abc123, "abc123")
end
