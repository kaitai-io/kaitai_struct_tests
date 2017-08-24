local luaunit = require("luaunit")

require("expr_io_pos")

TestExprIoPos = {}

function TestExprIoPos:test_expr_io_pos()
    local r = ExprIoPos:from_file("src/expr_io_pos.bin")

    luaunit.assertEquals(r.substream1.my_str, "CURIOSITY")
    luaunit.assertEquals(r.substream1.body, "\x11\x22\x33\x44")
    luaunit.assertEquals(r.substream1.number, 0x42)

    luaunit.assertEquals(r.substream2.my_str, "KILLED")
    luaunit.assertEquals(r.substream2.body, "a cat")
    luaunit.assertEquals(r.substream2.number, 0x67)
end
