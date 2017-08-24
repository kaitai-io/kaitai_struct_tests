local luaunit = require("luaunit")

require("default_endian_expr_inherited")

TestDefaultEndianExprInherited = {}

function TestDefaultEndianExprInherited:test_default_endian_expr_inherited()
    local r = DefaultEndianExprInherited:from_file("src/endian_expr.bin")

    luaunit.assertEquals(r.docs[1].indicator, "II")
    luaunit.assertEquals(r.docs[1].main.insides.some_int, 0x42)
    luaunit.assertEquals(r.docs[1].main.insides.more.some_int1, 0x4200)
    luaunit.assertEquals(r.docs[1].main.insides.more.some_int2, 0x42)
    luaunit.assertEquals(r.docs[1].main.insides.more.some_inst, 0x42)

    luaunit.assertEquals(r.docs[2].indicator, "MM")
    luaunit.assertEquals(r.docs[2].main.insides.some_int, 0x42)
    luaunit.assertEquals(r.docs[2].main.insides.more.some_int1, 0x42)
    luaunit.assertEquals(r.docs[2].main.insides.more.some_int2, 0x4200)
    luaunit.assertEquals(r.docs[2].main.insides.more.some_inst, 0x42000000)

    luaunit.assertEquals(r.docs[3].indicator, "XX")
    luaunit.assertEquals(r.docs[3].main.insides.some_int, 0x42)
    luaunit.assertEquals(r.docs[3].main.insides.more.some_int1, 0x42)
    luaunit.assertEquals(r.docs[3].main.insides.more.some_int2, 0x4200)
    luaunit.assertEquals(r.docs[3].main.insides.more.some_inst, 0x42000000)
end
