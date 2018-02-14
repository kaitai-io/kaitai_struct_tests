local luaunit = require("luaunit")

require("default_endian_expr_is_be")

TestDefaultEndianExprIsBe = {}

function TestDefaultEndianExprIsBe:test_default_endian_expr_is_be()
    local r = DefaultEndianExprIsBe:from_file("src/endian_expr.bin")

    -- LE
    luaunit.assertEquals(r.docs[1].indicator, "\x49\x49")
    luaunit.assertEquals(r.docs[1].main.some_int, 0x42)
    luaunit.assertEquals(r.docs[1].main.some_int_be, 0x42)
    luaunit.assertEquals(r.docs[1].main.some_int_le, 0x42)

    luaunit.assertEquals(r.docs[1].main.inst_int, 0x42)
    luaunit.assertEquals(r.docs[1].main.inst_sub.foo, 0x42)

    -- BE
    luaunit.assertEquals(r.docs[2].indicator, "\x4d\x4d")
    luaunit.assertEquals(r.docs[2].main.some_int, 0x42)
    luaunit.assertEquals(r.docs[2].main.some_int_be, 0x42)
    luaunit.assertEquals(r.docs[2].main.some_int_le, 0x42)

    luaunit.assertEquals(r.docs[2].main.inst_int, 0x42000000)
    luaunit.assertEquals(r.docs[2].main.inst_sub.foo, 0x42000000)

    -- Weird => LE
    luaunit.assertEquals(r.docs[3].indicator, "\x58\x58")
    luaunit.assertEquals(r.docs[3].main.some_int, 0x42000000)
    luaunit.assertEquals(r.docs[3].main.some_int_be, 0x42)
    luaunit.assertEquals(r.docs[3].main.some_int_le, 0x42)

    luaunit.assertEquals(r.docs[3].main.inst_int, 0x42)
    luaunit.assertEquals(r.docs[3].main.inst_sub.foo, 0x42)
end
