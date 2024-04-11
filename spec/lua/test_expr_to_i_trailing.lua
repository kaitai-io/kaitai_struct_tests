-- Autogenerated from KST: please remove this line if doing any edits by hand!

local luaunit = require("luaunit")

require("expr_to_i_trailing")

TestExprToITrailing = {}

function TestExprToITrailing:test_expr_to_i_trailing()
    local r = ExprToITrailing:from_file("src/term_strz.bin")

    luaunit.assertErrorMsgMatches(".+: ConversionError", function()
        local _ = r.to_i_r10
    end)
    luaunit.assertEquals(r.to_i_r16, 152517308)
    luaunit.assertErrorMsgMatches(".+: ConversionError", function()
        local _ = r.to_i_garbage
    end)
end
