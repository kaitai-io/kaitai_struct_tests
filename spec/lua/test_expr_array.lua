local luaunit = require("luaunit")

require("expr_array")

TestExprArray = {}

function TestExprArray:test_expr_array()
    local r = ExprArray:from_file("src/expr_array.bin")

    luaunit.assertEquals(r.aint_size, 4)
    luaunit.assertEquals(r.aint_first, 7657765)
    luaunit.assertEquals(r.aint_last, 16272640)
    luaunit.assertEquals(r.aint_min, 49185)
    luaunit.assertEquals(r.aint_max, 1123362332)

    luaunit.assertEquals(r.afloat_size, 3)
    luaunit.assertEquals(r.afloat_first, -2.6839530254859364e-121)
    luaunit.assertEquals(r.afloat_last, -1.1103359815095273e-175)
    luaunit.assertEquals(r.afloat_min, -8.754689149998834e+288)
    luaunit.assertEquals(r.afloat_max, -1.1103359815095273e-175)

    luaunit.assertEquals(r.astr_size, 3)
    luaunit.assertEquals(r.astr_first, "foo")
    luaunit.assertEquals(r.astr_last, "baz")
    luaunit.assertEquals(r.astr_min, "bar")
    luaunit.assertEquals(r.astr_max, "foo")
end
