-- Autogenerated from KST: please remove this line if doing any edits by hand!

local luaunit = require("luaunit")

require("expr_sizeof_type_1")

TestExprSizeofType1 = {}

function TestExprSizeofType1:test_expr_sizeof_type_1()
    local r = ExprSizeofType1:from_file("src/fixed_struct.bin")

    luaunit.assertEquals(r.sizeof_block, ((1 + 4) + 2) + 4)
    luaunit.assertEquals(r.sizeof_subblock, 4)
end
